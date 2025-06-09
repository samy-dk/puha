use clap::{Parser, Subcommand};
use puha_lib::{Item, Space};

/// Command line interface for managing spaces and items.
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the file storing the space tree
    #[arg(short, long, default_value = "space.json")]
    file: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new root space
    NewRoot { name: String },

    /// Show a space and all of its children
    ShowTree { name: Option<String> },

    /// Add an item to a space
    AddItem {
        space: String,
        item: String,
        description: String,
    },

    /// Add a space to another space
    AddSpace {
        parent: String,
        child: String,
    },

    /// List all items in a space
    ListItems { space: String },

    /// List all items and spaces in a space (one level)
    List { space: String },

    /// Move one or more items to a space
    MoveItems {
        from: String,
        to: String,
        items: Vec<String>,
    },

    /// Move a space and all its children to another space
    MoveSpace { space: String, to: String },

    /// Edit an item's name and/or description
    EditItem {
        space: String,
        item: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },

    /// Rename a space
    EditSpace { space: String, new_name: String },

    /// Delete an item from a space
    DeleteItem { space: String, item: String },

    /// Delete a child space and move its items to the parent
    DeleteSpace { parent: String, space: String },
}

fn print_tree(space: &Space, indent: usize) {
    let padding = "  ".repeat(indent);
    println!("{}{}", padding, space.name());
    for item in space.items() {
        println!("{}  - {}", padding, item.name());
    }
    for child in space.spaces() {
        print_tree(child, indent + 1);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::NewRoot { name } => {
            let root = Space::builder().name(name).root(true).build();
            root.save_to_file(cli.file)?;
        }
        Commands::ShowTree { name } => {
            let root = Space::from_file(&cli.file)?;
            let target = if let Some(n) = name {
                root.find_space(&n).ok_or("space not found")?
            } else {
                &root
            };
            print_tree(target, 0);
        }
        Commands::AddItem {
            space,
            item,
            description,
        } => {
            let mut root = Space::from_file(&cli.file)?;
            let target = root
                .find_space_mut(&space)
                .ok_or("space not found")?;
            let item = Item::builder().name(item).description(description).build();
            target.add_item(item);
            root.save_to_file(cli.file)?;
        }
        Commands::AddSpace { parent, child } => {
            let mut root = Space::from_file(&cli.file)?;
            let target = root
                .find_space_mut(&parent)
                .ok_or("space not found")?;
            let new_space = Space::builder().name(child).build();
            target.add_space(new_space);
            root.save_to_file(cli.file)?;
        }
        Commands::ListItems { space } => {
            let root = Space::from_file(&cli.file)?;
            let target = root.find_space(&space).ok_or("space not found")?;
            for item in target.items() {
                println!("{}", item.name());
            }
        }
        Commands::List { space } => {
            let root = Space::from_file(&cli.file)?;
            let target = root.find_space(&space).ok_or("space not found")?;
            for item in target.items() {
                println!("item: {}", item.name());
            }
            for sp in target.spaces() {
                println!("space: {}", sp.name());
            }
        }
        Commands::MoveItems { from, to, items } => {
            let mut root = Space::from_file(&cli.file)?;
            let mut removed = Vec::new();
            {
                let source = root
                    .find_space_mut(&from)
                    .ok_or("source space not found")?;
                for name in &items {
                    if let Some(item) = source.remove_item(name) {
                        removed.push(item);
                    }
                }
            }
            let dest = root
                .find_space_mut(&to)
                .ok_or("destination space not found")?;
            for item in removed {
                dest.add_item(item);
            }
            root.save_to_file(cli.file)?;
        }
        Commands::MoveSpace { space, to } => {
            let mut root = Space::from_file(&cli.file)?;
            let moved = root.remove_space(&space).ok_or("space not found")?;
            let dest = root
                .find_space_mut(&to)
                .ok_or("destination space not found")?;
            dest.add_space(moved);
            root.save_to_file(cli.file)?;
        }
        Commands::EditItem {
            space,
            item,
            name,
            description,
        } => {
            let mut root = Space::from_file(&cli.file)?;
            let target = root
                .find_space_mut(&space)
                .ok_or("space not found")?;
            let itm = target
                .find_item_mut(&item)
                .ok_or("item not found")?;
            if let Some(n) = name {
                itm.set_name(n);
            }
            if let Some(d) = description {
                itm.set_description(d);
            }
            root.save_to_file(cli.file)?;
        }
        Commands::EditSpace { space, new_name } => {
            let mut root = Space::from_file(&cli.file)?;
            let target = root
                .find_space_mut(&space)
                .ok_or("space not found")?;
            target.set_name(new_name);
            root.save_to_file(cli.file)?;
        }
        Commands::DeleteItem { space, item } => {
            let mut root = Space::from_file(&cli.file)?;
            let target = root
                .find_space_mut(&space)
                .ok_or("space not found")?;
            target
                .remove_item_local(&item)
                .ok_or("item not found")?;
            root.save_to_file(cli.file)?;
        }
        Commands::DeleteSpace { parent, space } => {
            let mut root = Space::from_file(&cli.file)?;
            let parent_space = root
                .find_space_mut(&parent)
                .ok_or("parent space not found")?;
            let removed = parent_space
                .remove_direct_space(&space)
                .ok_or("space not found")?;
            let items = removed.collect_items();
            for item in items {
                parent_space.add_item(item);
            }
            root.save_to_file(cli.file)?;
        }
    }

    Ok(())
}

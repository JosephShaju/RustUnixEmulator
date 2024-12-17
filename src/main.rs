use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Color, Print, Stylize},
    terminal::{self, Clear, ClearType},
};
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{stdout, Write};

fn main() -> crossterm::Result<()> {
    // Set emulator's working directory to the home directory
    if let Err(e) = set_to_home_directory() {
        eprintln!("Failed to set home directory: {}", e);
        return Ok(());
    }

    let mut stdout = stdout();

    // Enter raw mode
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut command_buffer = String::new();
    let mut output_lines: VecDeque<String> = VecDeque::new();
    const MAX_OUTPUT_LINES: usize = 20;

    loop {
        // Clear the screen
        queue!(stdout, Clear(ClearType::All))?;

        // Render the welcome message
        queue!(
            stdout,
            MoveTo(0, 0),
            Print("Welcome to the Unix Emulator".with(Color::Green)),
            MoveTo(0, 1),
            Print("------------------------------")
        )?;

        // Render Command Outputs
        for (index, line) in output_lines.iter().enumerate() {
            queue!(stdout, MoveTo(0, (index + 2) as u16), Print(line))?;
        }

        // Get the current working directory
        let current_dir = env::current_dir()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| "Unknown Directory".to_string());

        // Position Input Prompt Below Last Output
        let input_position = output_lines.len() as u16 + 2;
        queue!(
            stdout,
            MoveTo(0, input_position),
            Print(format!("> {} {}", current_dir, command_buffer).with(Color::Cyan))
        )?;

        stdout.flush()?;

        // Handle input
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                    command_buffer.push(c);
                }
                KeyCode::Backspace => {
                    command_buffer.pop();
                }
                KeyCode::Enter => {
                    if !command_buffer.trim().is_empty() {
                        let response = handle_command(&command_buffer, &mut output_lines);
                        if output_lines.len() >= MAX_OUTPUT_LINES {
                            output_lines.pop_front();
                        }
                        output_lines.push_back(format!("> {} {}", current_dir, command_buffer));
                        output_lines.push_back(response);
                        command_buffer.clear();
                    }
                }
                KeyCode::Esc => {
                    quit_terminal(&mut stdout)?;
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

/// Handles the execution of commands entered by the user.
fn handle_command(command: &str, output_lines: &mut VecDeque<String>) -> String {
    let mut parts = command.split_whitespace();
    let cmd = parts.next().unwrap_or("");

    match cmd {
        "ls" => list_directory(),
        "pwd" => current_directory(),
        "cat" => {
            let file_name = parts.next().unwrap_or("");
            read_file(file_name)
        }
        "echo" => {
            let args: Vec<&str> = parts.collect();
            echo_command(args)
        }
        "touch" => {
            let file_name = parts.next().unwrap_or("");
            let content = parts.collect::<Vec<&str>>().join(" ");
            create_file(file_name, &content)
        }
        "clear" => {
            if let Err(e) = clear_screen(&mut stdout(), output_lines) {
                format!("Error clearing screen: {}", e).with(Color::Red).to_string()
            } else {
                String::new() 
            }
        }
        "mkdir" => {
            let dir_name = parts.next().unwrap_or("");
            create_directory(dir_name)
        }
        "rm" => {
            let file_name = parts.next().unwrap_or("");
            delete_file(file_name)
        }
        "rmdir" => {
            let dir_name = parts.next().unwrap_or("");
            remove_directory(dir_name)
        }
        "cd" => {
            let dir_name = parts.next().unwrap_or("");
            change_directory(dir_name)
        }
        "exit" => {
            quit_terminal(&mut stdout()).unwrap();
            std::process::exit(0);
        }
        _ => format!("Unknown command: {}", cmd).with(Color::Red).to_string(),
    }
}

/// Clears the screen and resets the output buffer.
fn clear_screen(stdout: &mut std::io::Stdout, output_lines: &mut VecDeque<String>) -> crossterm::Result<()> {
    output_lines.clear();
    execute!(
        stdout,
        Clear(ClearType::All), 
        MoveTo(0, 0),             
        Print("Screen Cleared".with(Color::Yellow)), 
        Print("\n")               
    )?;
    stdout.flush()?;
    Ok(())
}

/// Lists the contents of the current directory.
fn list_directory() -> String {
    match fs::read_dir(".") {
        Ok(entries) => {
            let mut results: Vec<String> = entries
                .filter_map(|entry| {
                    entry.ok().map(|e| e.file_name().to_string_lossy().to_string())
                })
                .collect();

            results.sort();

            results.join("\n")
        }
        Err(e) => format!("Error: {}", e).with(Color::Red).to_string(),
    }
}

/// Returns the current working directory.
fn current_directory() -> String {
    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(e) => format!("Error: {}", e).with(Color::Red).to_string(),
    }
}

/// Reads the content of a file.
fn read_file(file_name: &str) -> String {
    if file_name.is_empty() {
        return "Error: File name is required.".with(Color::Red).to_string();
    }
    match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(e) => format!("Error reading file '{}': {}", file_name, e).with(Color::Red).to_string(),
    }
}

/// Creates a new file and optionally writes content to it.
fn create_file(file_name: &str, content: &str) -> String {
    if file_name.is_empty() {
        return "Error: File name is required.".with(Color::Red).to_string();
    }

    let sanitized_content = content.trim_matches('"');

    match File::create(file_name) {
        Ok(mut file) => {
            if !sanitized_content.is_empty() {
                if let Err(e) = writeln!(file, "{}", sanitized_content) {
                    return format!("Error writing to file '{}': {}", file_name, e).with(Color::Red).to_string();
                }
            }
            format!("File '{}' created.", file_name).with(Color::Green).to_string()
        }
        Err(e) => format!("Error creating file '{}': {}", file_name, e).with(Color::Red).to_string(),
    }
}

/// Creates a new directory.
fn create_directory(dir_name: &str) -> String {
    if dir_name.is_empty() {
        return "Error: Directory name is required.".with(Color::Red).to_string();
    }
    match fs::create_dir(dir_name) {
        Ok(_) => format!("Directory '{}' created.", dir_name).with(Color::Green).to_string(),
        Err(e) => format!("Error creating directory '{}': {}", dir_name, e).with(Color::Red).to_string(),
    }
}

/// Deletes a file.
fn delete_file(file_name: &str) -> String {
    if file_name.is_empty() {
        return "Error: File name is required.".with(Color::Red).to_string();
    }
    match fs::remove_file(file_name) {
        Ok(_) => format!("File '{}' deleted.", file_name).with(Color::Green).to_string(),
        Err(e) => format!("Error deleting file '{}': {}", file_name, e).with(Color::Red).to_string(),
    }
}

/// Removes an empty directory.
fn remove_directory(dir_name: &str) -> String {
    if dir_name.is_empty() {
        return "Error: Directory name is required.".with(Color::Red).to_string();
    }
    match fs::remove_dir(dir_name) {
        Ok(_) => format!("Directory '{}' removed.", dir_name).with(Color::Green).to_string(),
        Err(e) => format!("Error removing directory '{}': {}", dir_name, e).with(Color::Red).to_string(),
    }
}

/// Changes the current directory.
fn change_directory(dir_name: &str) -> String {
    if dir_name.is_empty() {
        return "Error: Directory name is required.".with(Color::Red).to_string();
    }
    match env::set_current_dir(dir_name) {
        Ok(_) => format!("Changed directory to '{}'.", dir_name).with(Color::Green).to_string(),
        Err(e) => format!("Error changing directory to '{}': {}", dir_name, e).with(Color::Red).to_string(),
    }
}

/// Handles the `echo` command to display user-provided text.
fn echo_command(args: Vec<&str>) -> String {
    args.join(" ") // Join all arguments with a space
}

/// Quits the terminal emulator and restores the terminal to its normal state.
fn quit_terminal(stdout: &mut std::io::Stdout) -> crossterm::Result<()> {
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?; // Exit raw mode
    println!("Exiting Unix Emulator. Goodbye!");
    Ok(())
}

/// Sets the emulator's working directory to the home directory.
fn set_to_home_directory() -> std::io::Result<()> {
    if let Some(home_dir) = dirs::home_dir() {
        env::set_current_dir(home_dir)?;
    } else {
        eprintln!("Home directory not found.");
    }
    Ok(())
}

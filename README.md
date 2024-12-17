# Project Description - Rust-based Terminal Emulator

**Joseph Thaliath**  
**Student ID:** 1003604209  

---

## **Project Description: Unix Emulator**

The Unix Emulator is a Rust-based terminal application designed to replicate essential Unix shell functionalities. It features a single window, basic command input, and color-coded output, focusing on simplicity and core functionality. The project introduces fundamental concepts in systems programming, command-line interaction, and Rust’s performance and safety benefits. This makes it both an educational tool for learning Rust and a practical emulator for basic Unix-like operations.

---

## **Features**

### **File and Directory Operations**
- **Create files:**  
  `touch <filename> "<text>"` — Create a file with optional text entry. Text should be enclosed in double quotes, but the double quotes will not be written into the file.  
- **Create directories:**  
  `mkdir <directory_name>` — Create a directory.
- **Remove files and directories:**  
  `rm <filename>` — Remove a file.  
  `rmdir <directory_name>` — Remove a directory.

### **Navigation Commands**
- **Change directories:**  
  `cd <directory_name>` — Move to a specified directory.
- **Display current directory:**  
  `pwd` — Show the current working directory.
- **List files and directories:**  
  `ls` — Display contents of the current directory.  
  *(Note: The spacing and padding for the `ls` command could not be fully resolved.)*

### **File Content Management**
- **Read files:**  
  `cat <filename>` — Display the content of a file.  
- **Interactive file creation:**  
  `touch <filename>` — Enter content directly during file creation.

### **General Commands**
- **Output text:**  
  `echo <message>` — Display a custom message.  
- **Clear terminal:**  
  `clear` — Clear the terminal screen.

### **Interactive Shell Features**
- Displays the **current working directory** in the command prompt.
- Maintains a **scrollable output log** for command history.
- Exit the emulator gracefully using `exit` or pressing **Esc**.

---

## **How to Operate the Emulator**

### **Launch the Emulator**
Run the following command from the project directory to start the emulator in release mode:  

```bash
cargo run --release

## **Video Demonstration**  
For a complete walkthrough of the emulator's features and usage, watch the video demo:  

[**Video Demo Link**](https://www.dropbox.com/scl/fi/zbof2hffrfjurzr0mrqnh/Video-Demo-ECE-1724.mov?rlkey=pil1sl0myysywkxbuhmws4ad3&st=yuo3i5uq&dl=0)


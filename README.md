# Tauri + React

Text Editor for Windows:

1. Load a file
2. Modify its content
3. Save the file

The project was creating during a 2024 Rust Workshop, for learning
Tauri framework. The implementation idea consists in creating a
text box (textarea in HTML) that is associated with a variable modification
using states (the content as a string). When clicking on load button, the
associated "load_file" react function calls Rust "load_file" which opens
a tab where the user can select the file from his computer, functionality
implemented with dialog over the AppHandle app object. After opening the
file with fs::read_to_string(file) and checking for errors, the content
of the file is returned as String and then immediately assigned to
textareaVal in React, so that the content will appear in the editor.
Every modification is logged by changing the value with setTextareaVal.
When clicking on save button, the react save_file function is called
(with textareaVal that represent the edited content) which calls
save_file from Rust. The file in which the content will be saved
will be again chosen by the user via the file system window UI (again
dialog), and the content is just written in that file using fs::write.

## Recommended IDE Setup
path: C:\Users\User\Desktop\Tauri_app\rustpad

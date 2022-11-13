# mini_grep

This is a project that I am working on as a person learning rust. This is a comand line application that will search for a string in multiple files.
It is made completely in rust.

### Usage
If you would like this program, you will have to download then compile ths source code then compile it your self. 
For better use add the binary created to the PATH variable. I might add a release later but for now you will have to download the source code.

Mini Grep will search for a string in a list of files and return all lines that have the string.

To use type 'minigrep (string-to-search) (optional-case_sensitive) (files-to-search-in-seperated-by-commas)'

You do not need to provide a value for the case-sensitive argument. 
By default the search is not case sensitive. Add the number 1 after the search term to make it case sensitive.

Usage example: 'mini_grep "hello" "poem.txt", "poem2.txt"'

Quotations are only required for multi word strings or files.
Make sure that the file passed in has valid UTF-8 characters and exist in the current directory.
For help type 'mini-grep help'

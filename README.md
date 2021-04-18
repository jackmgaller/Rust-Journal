# Rust-Journal
A simple journaling system, written in Rust.

**You must create a `meta` folder and a `data` folder in the root directory for Rust-Journal to work![1]**

## Documentation

```
--template : Creates a template file in the root directory.

--store FILEPATH : Stores a file as an entry in Rust-Journal.
--add-tags FILEPATH TAG1,TAG2,ETC : Adds tags to an existing entry.

--print-all : Prints all entries.
--print FILEPATH : Prints a specified entry.
```

[1] .gitignore files don't allow empty folders, keeping them would mean pushing any changes to a fork of this project would also require pushing all of your sensitive journal entries
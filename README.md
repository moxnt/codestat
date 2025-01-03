## codestat
A command line utilityto view the most used programming languages in a project. Languages are decided using file extensions and usage is currently counted by line count.

> [!NOTE]
> There are still some important improvements that need to be made. This utility does NOT check .gitignore so it will become very slow if it gets into a heavy directory (for example node_modules)

### Usage
```
./codestat [path to directory]
./codestat # By default it checks ./src (Realtive to current working directory)
./codestat ../app/src #.. also works
```

### Instalation
1. Build from source
2. (Optional) Place extensions.json in a res directory next to the program.
3. Run the program.

## codestat
A command line utility to view the most used programming languages in a project. Languages are decided using file extensions and usage is currently counted by line count.

> [!NOTE]
> There are still some important improvements that need to be made. This utility does NOT check `.gitignore` so it will become very slow if it gets into a heavy directory (for example node_modules)

*If it starts crawling the node modules directory it may sometimes read so much node_modules code, that it overpowers your own*
![Output example for a website directory that contained a node_modules folder](./github/example2.png)

### Usage
```
./codestat [path to directory]
./codestat # By default it checks ./src (Realtive to current working directory)
./codestat ../app/src #.. also works
```
![Output example for a website directory that contained only html+css sites](./github/example1.png)

### Instalation
1. Build from source
2. (Optional) Place extensions.json in a res directory next to the program.
3. Run the program.

### Changes
- [x] Colors
- [ ] Skip directories and files that match `.gitignore`
- [ ] Read metadata instead of lines
- [ ] More custom colors for languages

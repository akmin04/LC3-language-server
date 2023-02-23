Andy Min (akmin2), 2023-02-22

# LC-3 Language Server (LC3LS)

LC3LS consists of two main parts: a language server and a VSCode extension.

## Language Server

The main part of the project will be to implement the language server protocol for the LC-3 language. It will feature the following:

 * Autocompletion (instruction names, label names, trap vectors, etc)
 * Goto definitions (navigating to declarations of labels)
 * Basic static checks
     * Invalid instructions (instructions that don’t exist)
     * Invalid labels (labels that don’t exist)
     * Invalid instruction arguments (incorrect number or type of arguments)
     * Invalid immediate values (integers outside of the range of an instruction argument)

## VSCode extension
The VSCode extension will provide an interface for LC3LS to be used in VSCode. In addition to implementing the language server features into VSCode, the extension will also add syntax highlighting.

## Milestones
 * [3/26 - Progress report 1] Implement the language server, consisting of the lexer/parser for LC-3 and the various language features
 * [4/23 - Progress report 2] Implement the VSCode extension with the language server’s features
 * [5/12 - Code demo/final report] Implement extra features into the VSCode extension such as syntax highlighting


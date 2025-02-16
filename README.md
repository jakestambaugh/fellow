# Fellow

A programming language designed for coding interviews

## Why

Why make a programming language: I want to make a programming language to teach myself how to make one! I think that's a pretty good reason in its own right.

The more interesting "why" might be "Why focus on technical interviews": I want to make a programming language by starting with a unique set of constraints and values. I think that programming languages can only succeed and fail when compared to their own stated values. Many programming languages state that they target professional programmers and are designed for production software. Many programming languages state which layer of the stack they are intending to be powerful with. Technical interviews are a new environment and let me explore new constraints.

### Technical Interviews are bad 

I agree! The standard FAANG "whiteboard" interview and its like are not a great measure of programming ability. I think that they are most able to measure stress.

I specifically wanted to make a programming language for technical interviews because it's a specific and constrained environment for programming. I think it's interesting to explore what a programming language looks like with different constraints to the many "general-purpose" languages in the ecosystem.

## What

### Tooling

To start, Fellow will be interpreted using a CLI tool `fellow`. It will take a `.fellow` file path as the first argument to stdin, which it will then interpret.

### Language Features

#### Comments are implicit

This might end up being a dumb idea, but in-line text next to code is a valuable tool for communication. Jupiter notebooks accomplish this in a Rich-Text environment, and Literate Programming attempts to promote prose the default (while making code block explicit with `<<>>` chunks).

#### Gradually typed

Gradual typing will help programmers in interviews to refine rough drafts of their solutions.

#### No garbage collection

I want programmers to be able to express their understanding of memory management, while not worrying about it in cases where memory management isn't the goal. Also, memory leaks don't matter since this code will only live for a short while. Being able to express deallocation will be useful, but not necessary.

#### No shadowing or redeclaration

This is where my personal preferences start to show, but I think that during a technical interview it can be very easy to reuse the name of a variable in a different scope. Preventing accidental shadowing is helpful, because its easy to fix (simply rename) 

## How

#### Comments are implicit

Comments will be represented in the AST for debugging, and if a line starts with a non-recognized expression it will be treated as a comment.

## Notes and resources

https://ntietz.com/blog/you-should-make-a-new-terrible-programming-language/

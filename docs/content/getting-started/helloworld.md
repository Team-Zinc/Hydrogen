---
title: "Hello, world!"
date: 2020-09-02T23:40:40-06:00
draft: true
weight: 3
---

### Hello, world!
Let's build a simple hello world example project in C. First we define our ```hy.yml``` file.

#### Example
```yaml
name: hello
desc: A hello, world project built with Hydrogen.
authors: [Milo Banks]
version:
  - major: 0
  - minor: 0
  - patch: 1

projects:
  - hello:
    authors: [Milo Banks]
    kind: console
    version:
      major: 0
      minor: 0
      patch: 1

    files: [src/main.c]
```

As you can see, it's pretty self explanitory. One thing to note is that Hydrogen uses workspaces to encapsulate projects, just because of how usefull this can be sometimes.
It's not really clear that we are creating a workspace, but we are. We are setting the name, the description, the authors, and the version. Pretty simple, right?
Now we get into more complex teratory. We create a project. A project in Hydrogen is a unit (or project) that gets compiled together to produce a single file (like an executable, or a static library). We add a project called hello to the list of projects, set the authors, the kind of application (either console, gui, sharedlib, or staticlib), the version, and the files. Pretty neat, huh?
While we're at it, let's write our hello world C program.
```C
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    printf("Hello, world!\n");

    return EXIT_SUCCESS;
}
```

If we now run ```hy build``` (which automatically runs ```hy configure``` and the like), it should produce a binary called hello. Let's run it.
```
Hello, world!
```

Success!

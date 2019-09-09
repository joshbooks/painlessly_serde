The idea of this package is to provide (you guessed it) painless serde.
What do I mean by that?

A serialization framework that provides backwards, forwards, and
sideways compatibility. Strong typing. Class Structures.

OK, so what's the catch?

You write your own migrations in a DSL. I know, I know. But you know
why and you know it's the right way to do it. Also one class/struct
per file. Because I said so.

How do you handle versioning?

I dream of sort of riding along with git, but that means being a little 
more opinionated about the structure of downstream projects than I 
really want to be. So for now the top level subdir is full of semver versions
and a migrations dir with migrations between the semver versions. When you 
want to cut a new version all you got to do is copy the latest version to
a new folder with the semver of the new version and migrations that go 
backward and forward.

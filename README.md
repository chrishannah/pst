# pst

A simple command-line tool to publish short posts to Micro.blog.

This is partially a personal project to learn Rust, but also to build something
that I will use myself. If other people find it useful in any capacity,
then that is a bonus.

## installation

### cargo

You can install pst directly from cargo. Just use the following command:

```
cargo install pst
```

### homebrew

pst is available from homebrew, however you'll need to use a custom tap.
Although it's still pretty easy:

```
brew tap chrishannah/pst
brew install pst
```

## usage 

Using pst is pretty simple.

The command is `pst`, you then need to say if it's a draft or not, by using
`draft` or `post`, and then you write your micro post.

So to write a draft post, you would do something like this:

```
pst draft "Hello, world. This is a draft post"
```

Or to publish it directly:

```
pst post "only losers draft posts"
```

Keep in mind, that if you use anything other than `draft` or `post`, it will
default to a draft post.

## configuration

pst is configured using a rather simple json file, stored in your home
directory, under `.config/pst/config.json`. It only needs to contain your
Micro.blog app token in the following format:

```json
{
    "token": "micro.blog token"
}
```

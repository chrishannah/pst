# pst

A simple command-line tool to publish short posts to Micro.blog.

This is partially a personal project to learn Rust, but also to build something
that I will use myself. If other people find it useful in any capacity,
then that is a bonus.

## usage 

## configuration

pst is configured using a rather simple json file, stored in your home
directory, under `.config/pst/config.json`. It only needs to contain your
Micro.blog app token in the following format:

```json
{
    "token": "micro.blog token"
}
```

## functionality

- [x] publish drafts to Micro.blog
- [x] configure app token in config.json
- [x] better output when publishing post
- [x] get config from file relative to home directory
- [ ] usage instructions
- [ ] control post status via argument
- [ ] maybe support input from stdin

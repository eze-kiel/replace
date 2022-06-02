# replace

Small CLI utility that simplify `sed 's//g'`.

## Usage

The basic usage of `replace` is:

```
replace expr:replace [filename]
```

Let's imagine that we have the following text file called `quote.txt`:

```
Make it correct, make it clear, make it concise, make it fast. In that order.
- Wes Dyer
```

We cat use `replace` directly by providing it the filename:

```
$ replace correct:perfect quote.txt
Make it perfect, make it clear, make it concise, make it fast. In that order.
- Wes Dyer
```

`replace` also works with pipes:

```
$ cat quote.txt | replace correct:perfect
Make it perfect, make it clear, make it concise, make it fast. In that order.
- Wes Dyer
```

## Known limitations

`replace` currently supports only one pattern.

## License

MIT
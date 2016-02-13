# copper.rs

An extremely simpler application server, based on [nickel.rs](https://github.com/nickel-org/nickel.rs).

## The idea

- To experiment with Rust and learn more about it by practical experience.
- To get some experience with nickel.rs.
- To see how well it would work to build a general-purpose application server in nickel.rs, with support for:
  - _Hot code reloading_ (i.e. when changing a source file in the app, the app server should automatically use the new version).
  - Some form of trivial _"routing"_ (i.e. be able to say that "file/method foo should be able to handle GET requests for URI bar")

 ## Hot code reloading

 Hot code reloading has not been implemented. There are [a few options](http://stackoverflow.com/a/35380639/227779), but I'm not sure if any of them properly supports _unloading_ a shared lib. Also, some of the options don't seem to handle Rust naming schemes properly, making them more awkward to use.

 As a simpler approach: maybe we could just ignore that idea for now, and use a [guard](https://github.com/guard/guard) based approach instead. I.e., when the _application_ logic changes, let Guard detect that and:

- recompile the code
  - kill the existing instance of the server
  - start it up anew

 If this is reasonably fast (say, within 2-3 seconds on an average-scale app), I guess it could be an acceptable compromise for now.

 Future options:

 - use proper code reloading by means of a shared library
 - don't handle code reloading, but instead use some form of "script" engine to allow the user to write the app logic in a script language instead.

   This has the advantage of likely being easy to support reloading of code quite easily, and a reasonably high level of productivity, but the disadvantage of maybe loosing some of Rust's advantages (i.e. strict typing, compile-time errors when making incomplete refactorings, etc etc.)

 ## Routing

 To be finalized/planned. The idea here would be to let the app "announce" in some form or another what routes it supports, and what class/method is handling that specific route.

## Contributing

For certain, pull requests and suggestions are welcome. This is just something I'm doing on my spare time so "expect no wonders".

## License

[MIT](https://opensource.org/licenses/mit-license.php)

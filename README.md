# Milf Nutters 🤌🏻

Hello! Why are you here?

Oh, I though so. You are here for a Rust program to generate some random phrases based on books you feed it but it actually does something you did not expect it to.

😬

## What does it do exactly?

Well, I dunno. You:
1. Give it a list of books,
2. Generate a graph of words,
3. Enter interactive mode,
4. Click enter till it generates a funny enough phrase that you send it to your friends thinking you are funny while they start to question your friendship.

## How do I use it?

### Compile stage

Start off by compiling the source. While in **src** folder, use either ``cargo build`` or ``cargo build --release`` if you want it to be faster (you probably want that option ngl).

After it all compiles, copy **libraries.json** file from root into the same directory that your compiled program is in. It will not work without it. So based on what you did, you either have to run these from the root dir:

```cp src/libraries.josn target/debug/libraries.json```

or

```cp src/libraries.josn target/release/libraries.json```

### Playing with it

After it's compiled, you want to start up the compiled program from the console. So do that.

Immediately you will see a console list of stuff you can do in it. there is a default library with URLs already so I can show you what to do to start it up:

``:build_graph`` - this will download files from the links in the library and save them in a folder to later read from (or load them if they are already downloaded). After that, it builds a graph of words. This can take some time on debug, probably faster on release version.

``:interactive`` - enter interactive mode. Here you eiter click **enter** to submit an empty string (program picks a starting word at random) or put in one word and click enter to generate a sentence starting with that word.

**Is it fun yet??**

### More options

Let me show you how you would usually go about creating your own library and managing URLs in it 🫡

We will create a library called **gibberish** and put a few books in it, then use that to generate our phrases.

1. Run ``:print_libraries`` - you can see there is one default library with 3 URLs from project Gutenberg. We will be using books from their site because they are free and it's the only thing that I wanted to use
2. Run ``:add_library gibberish`` - this will add an empty library. Verify that by running ``:print_libraries`` again
3. Run ``:set_library gibberish`` - you will see that on the top of the options you see **---CONSOLE OPTIONS "gibberish"/Not loaded)---** instead of **---CONSOLE OPTIONS "default"/Not loaded)---**. You have changed the default library you are using, letsgoooo!
4. Run ``:add_url https://www.gutenberg.org/files/2701/2701-h/2701-h.htm`` - adds new URL to the library you are using. Verify that by running ``:print_libraries``
5. Run ``:add_url https://www.gutenberg.org/files/37392/37392-h/37392-h.htm`` - adds another URL to the library
6. Run ``:add_url https://www.gutenberg.org/files/17476/17476-h/17476-h.htm`` - adds the last URL we wnted
7. Run ``:print_libraries`` - see your changes
8. Run ``:fetch_data`` - loads text from URLs, processes them and saves into a file
9. Run ``:build_graph`` - build a graph of words to use. At the end of this, you should see **---CONSOLE OPTIONS "gibberish"/Loaded)---**, indicating that the graph is loaded and ready to use
10. Run ``:interactive`` - let it speak gibberish by pressing enter. You could also type in ``dick`` and see what it tells you since we had **Moby Dick** in the list of books. Hah, so funny! 🥳🥳👏🍆

Here are some of the things I got:
> Equipped salesman to spend too great it is its sum of that not a coffin, mr.

> Chestnuts, nor will here& ldquo;
 it oddly dashed.

> Dick not only be a duel.

### Rest of the options

So first of all, not all of the options are yet available, but all that I listed above work. Removing libraries and URLs also works, so you can use that too. 

### Where do I find links to the books?

Good question fam! Let me show you 👩‍💻

1. Go to https://www.gutenberg.org/ebooks/search/?sort_order=random and pick or find a book
2. Once you find a book, click on it, you will see a list of sources to choose from. **This is important** - Click on the **Read this book online: HTML (as submitted)**
3. Copy the link form the page, for me it looks like that: https://www.gutenberg.org/files/50309/50309-h/50309-h.htm
4. Paste it into ``:add_url`` to add it to a library
5. Done

## Why, and how can I help

Idk and idk. This project is used for me to learn Rust. It's poorly written, you will probably encounter bugs or it will just crash if you give it something I havent mention in this readme.

If you have an idea about some new features, you can start a discusson. I will not be accepting PRs, just because. 

This will also probably add weird HTML characters to your output if source had some images or non-english letters but it just adds that shitpost flavour to it so yeah, it is a **feature**,not a bug.

## This is the end

Thank you to Project Gutenberg for providing a list of books in HTML5 that I can process for free, they are the real OGs.

Bye! 🎉



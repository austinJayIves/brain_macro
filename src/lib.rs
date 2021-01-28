#[macro_export]
macro_rules! brainfuck {
    (@start $($sym:tt)*) => {{
        type Cell = u8;
        const MAX_SIZE: usize = 30_000;

        fn brainfuck() -> ::std::io::Result<Vec<Cell>> {
            use ::std::io;
            use ::std::io::prelude::*;

            fn _err() -> io::Error {
                io::Error::new(
                    io::ErrorKind::Other,
                    String::from("No more input available"),
                )
            }

            fn _inc(a: &mut [Cell], i: usize) {
                a[i] = a[i].wrapping_add(1);
            }

            fn _dec(a: &mut [Cell], i: usize) {
                a[i] = a[i].wrapping_sub(1);
            }

            let _r = &mut io::stdin();
            let _w = &mut io::stdout();

            let mut _tape: Vec<Cell> = ::std::iter::repeat(0).take(MAX_SIZE).collect();
            let mut _idx = 0;
            {
                let _tape = &mut _tape;
                brainfuck!(@fuckery (_tape, _idx, _r, _w, _inc, _dec, _err); ($($sym)*));
            }

            Ok(_tape)
        }
        let _ = brainfuck();
    }};

    (@fuckery $sym:tt; ()) => {};

    // Rules
    (@fuckery ($tape:expr, $idx:expr, $r:expr, $w:expr, $inc:expr, $dec:expr, $err:expr); (> $($sym:tt)*)) => {
        $idx = ($idx + 1) % MAX_SIZE;
        brainfuck!(@fuckery ($tape, $idx, $r, $w, $inc, $dec, $err); ($($sym)*));
    };

    (@fuckery ($tape:expr, $idx:expr, $r:expr, $w:expr, $inc:expr, $dec:expr, $err:expr); (< $($sym:tt)*)) => {
        $idx = ($idx + MAX_SIZE - 1) % MAX_SIZE;
        brainfuck!(@fuckery ($tape, $idx, $r, $w, $inc, $dec, $err); ($($sym)*));
    };

    (@fuckery ($tape:expr, $idx:expr, $r:expr, $w:expr, $inc:expr, $dec:expr, $err:expr); (+ $($sym:tt)*)) => {
        $inc($tape, $idx);
        brainfuck!(@fuckery ($tape, $idx, $r, $w, $inc, $dec, $err); ($($sym)*));
    };

    (@fuckery ($tape:expr, $idx:expr, $r:expr, $w:expr, $inc:expr, $dec:expr, $err:expr); (- $($sym:tt)*)) => {
        $dec($tape, $idx);
        brainfuck!(@fuckery ($tape, $idx, $r, $w, $inc, $dec, $err); ($($sym)*));
    };

    (@fuckery ($tape:expr, $idx:expr, $r:expr, $w:expr, $inc:expr, $dec:expr, $err:expr); (. $($sym:tt)*)) => {
        $w.write_all(&$tape[$idx .. $idx + 1])?;
        brainfuck!(@fuckery ($tape, $idx, $r, $w, $inc, $dec, $err); ($($sym)*));
    };

    (@fuckery ($tape:expr, $idx:expr, $r:expr, $w:expr, $inc:expr, $dec:expr, $err:expr); (, $($sym:tt)*)) => {
        match $r.read(&$tape[$idx .. $idx + 1])? {
            // Unable to read a byte. We're fucked. Return an error
            Ok(0) => Err($err()),

            // Otherwise, return whether or not we were able to read a single byte
            v => v
        }?;
        brainfuck!(@fuckery ($tape, $idx, $r, $w, $inc, $dec, $err); ($($sym)*));
    };

    (@fuckery ($tape:expr, $idx:expr, $r:expr, $w:expr, $inc:expr, $dec:expr, $err:expr); ([ $($inner:tt)* ] $($tail:tt)*)) => {
        while $tape[$idx] != 0 {
            brainfuck!(@fuckery ($tape, $idx, $r, $w, $inc, $dec, $err); ($($inner)*));
        }
        brainfuck!(@fuckery ($tape, $idx, $r, $w, $inc, $dec, $err); ($($tail)*));
    };

    (@entry $($sym:tt)*) => {
        brainfuck!(@start $($sym)*)
    }

}

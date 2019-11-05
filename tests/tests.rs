use brainfrsck::prelude::*;
use rand::prelude::{Rng, thread_rng};

const QUICKSORT_BF: &'static str = r#">>+>>>>>,[>+>>,]>+[--[+<<<-]<[<+>-]<[<[-
>[<<<+>>>>+<-]<<[>>+>[->]<<[<]<-]>]>>>+<[[-]<[>+<-]<]>[[>>>]+<<<-<[<<[<<<]>>+>
[>>>]<-]<<[<<<]>[>>[>>>]<+<<[<<<]>-]]+<<<]+[->>>]>>]>>[.>>>]"#;

const SUM_BF: &'static str = r#",>,<[[->+<],]>."#;

const CELL_SIZE_BF: &'static str = r#"++++++++[>++++++++<-]>[<++++>-]+<[>-<[>+
+++<-]>[<++++++++>-]<[>++++++++<-]+>[>++++++++++[>+++++<-]>+.-.[-]<<[-]<->]<[>
>+++++++[>+++++++<-]>.+++++.[-]<<<-]]>[>++++++++[>+++++++<-]>.[-]<<-]<++++++++
+++[>+++>+++++++++>+++++++++>+<<<<-]>-.>-.+++++++.+++++++++++.<.>>.++.+++++++.
.<-.>>-.[[-]<]"#;

const SIERPINSKI_BF: &'static str = r#"++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>
>]>+[-<<<[->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<]>.>+[>>]>+]"#;

const SIERPINSKI_RESULT: &'static str = r#"                               *
                              * *
                             *   *
                            * * * *
                           *       *
                          * *     * *
                         *   *   *   *
                        * * * * * * * *
                       *               *
                      * *             * *
                     *   *           *   *
                    * * * *         * * * *
                   *       *       *       *
                  * *     * *     * *     * *
                 *   *   *   *   *   *   *   *
                * * * * * * * * * * * * * * * *
               *                               *
              * *                             * *
             *   *                           *   *
            * * * *                         * * * *
           *       *                       *       *
          * *     * *                     * *     * *
         *   *   *   *                   *   *   *   *
        * * * * * * * *                 * * * * * * * *
       *               *               *               *
      * *             * *             * *             * *
     *   *           *   *           *   *           *   *
    * * * *         * * * *         * * * *         * * * *
   *       *       *       *       *       *       *       *
  * *     * *     * *     * *     * *     * *     * *     * *
 *   *   *   *   *   *   *   *   *   *   *   *   *   *   *   *
* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
"#;

const REVERSE_BF: &'static str = r#",[>,]<[.[-]<]"#;

const HELLOWORLD_BF: &'static str = r#"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>
->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."#;

const FANCY_NUMBERS_BF: &'static str = r#">>>>+>+++>+++>>>>>+++[>,+>++++[>++++
<-]>[<<[-[->]]>[<]>-]<<[>+>+>>+>+[<<<<]<+>>[+<]<[>]>+[[>>>]>>+[<<<<]>-]+<+>>>-
[<<+[>]>>+<<<+<+<--------[<<-<<+[>]>+<<-<<-[<<<+<-[>>]<-<-<<<-<----[<<<->>>>+<
-[<<<+[>]>+<<+<-<-[<<+<-<+[>>]<+<<<<+<-[<<-[>]>>-<<<-<-<-[<<<+<-[>>]<+<<<+<+<-
[<<<<+[>]<-<<-[<<+[>]>>-<<<<-<-[>>>>>+<-<<<+<-[>>+<<-[<<-<-[>]>+<<-<-<-[<<+<+[
>]<+<+<-[>>-<-<-[<<-[>]<+<++++[<-------->-]++<[<<+[>]>>-<-<<<<-[<<-<<->>>>-[<<
<<+[>]>+<<<<-[<<+<<-[>>]<+<<<<<-[>>>>-<<<-<-]]]]]]]]]]]]]]]]]]]]]]>[>[[[<<<<]>
+>>[>>>>>]<-]<]>>>+>>>>>>>+>]<]<[-]<<<<<<<++<+++<+++[[>]>>>>>>++++++++[<<++++>
++++++>-]<-<<[-[<+>>.<-]]<<<<[-[-[>+<-]>]>>>>>[.[>]]<<[<+>-]>>>[<<++[<+>--]>>-
]<<[->+<[<++>-]]<<<[<+>-]<<<<]>>+>>>--[<+>---]<.>>[[-]<<]<]"#;

const FANCY_NUMBERS_0123_RESULT: &'static str = "      \
      /\\
       /\\
    /\\  /
     / 
   \\ \\/
    \\
/\\   
\\ \\
 \\/
";

/// Tests for the `ToBytes` trait
#[test]
fn to_bytes() -> Result<(), BrainfuckError> {
    assert_eq!(
        SIERPINSKI_RESULT.clone().to_bytes(),

        eval_string(
            SIERPINSKI_BF,

            None
        )?.to_bytes()
    );
    
    assert_eq!(
        "Hello World!".to_owned().chars().rev().collect::<String>().to_bytes(),

        eval_string(
            REVERSE_BF,

            Some("Hello World!".to_bytes())
        )?.to_bytes()
    );

    assert_eq!(
        "8 bit cells\n".to_bytes(),

        eval_string(
            CELL_SIZE_BF,

            None
        )?.to_bytes()
    );


    Ok(())
}

/// Test that the correct error is returned when it should be
#[test]
fn errors() -> Result<(), BrainfuckError> {
    
    // makes sure it returned an error and checks if the message is correct
    fn chk<ICantNameThisUnderscore>(res: Result<ICantNameThisUnderscore, BrainfuckError>, msg: &'static str) -> bool {
        res.is_err() && res.err().unwrap().msg() == msg.to_owned()
    }

    let missing_open = eval_string("+]", None);
    let missing_close = eval_string("[", None);
    let oom = eval_string("+[+>+]", None);

    assert!(chk(oom, "Attempt to increment data pointer past the end of memory"));
    assert!(chk(missing_open, "Mismatched jump operations (missing a `[`)"));
    assert!(chk(missing_close, "Mismatched jump operations (missing a `]`)"));

    Ok(())
}
   
/// A quicksort implentation I found online, kinda a stress test
#[test]
fn quicksort() -> Result<(), BrainfuckError> {
    fn is_sorted(v: &Vec<u8>) -> bool {
        let mut s = v.clone();
        s.sort();

        s == v.clone()
    }

    fn rand_byte() -> u8 {
        thread_rng().gen::<u8>()
    }

    // Test sort a bunch of random numbers
    assert!(
        is_sorted(
            &eval_string(
                QUICKSORT_BF,
                Some(
                    (0..1000)
                    .map(|_| rand_byte())
                    .collect()
                )
            )?.to_bytes()
        )
    );
    Ok(())
}

/// Test the sierpinski triangle thing i found online
#[test]
fn sierpinski() -> Result<(), BrainfuckError> {
    assert_eq!(
        eval_string(SIERPINSKI_BF, None)?.to_string(),
        SIERPINSKI_RESULT,
    );
    Ok(())
}

#[test]
fn sum() -> Result<(), BrainfuckError> {
    fn wrapping_sum(vals: &Vec<u8>) -> u8 {
        vals.clone().iter().fold(0, |a: u8, x: &u8| a.wrapping_add(*x))
    }

    // sum
    for _ in 0..10 {

        let vals: Vec<u8> = (0..8)
                    // This 1 is very important, if you can have 0s in your values, this sum (bf) won't work
                    .map(|_| thread_rng().gen_range(1, 32))
                    .collect();


        assert_eq!(
            eval_string(
                SUM_BF,
                Some(
                    vals.clone()
                )
            )?
                .to_vec()
                [0],
            wrapping_sum(
                &vals
            ),
        )

    }
    
    Ok(())
}

#[test]
fn helloworld() -> Result<(), BrainfuckError> {

    // Hello world
    assert_eq!(
        "Hello World!\n"
            .to_owned(),
        
        eval_string(
            HELLOWORLD_BF,
            None
        )?.to_string()
    );
    Ok(())
}

#[test]
fn fancynumbers() -> Result<(), BrainfuckError> {
    // Pretty numbers 
    assert_eq!(
        FANCY_NUMBERS_0123_RESULT
            .to_owned(),
        eval_string(
            FANCY_NUMBERS_BF,
            Some(
                "0123"
                    .to_owned()
                    .to_bytes()
                )
            )?
                .to_string()
        );
    Ok(())
}


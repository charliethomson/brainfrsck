use brainfrsck::error::BrainfuckError;

#[test]
fn foo_bar() -> Result<(), BrainfuckError> {
    use brainfrsck::prelude::*;

    let sum = r#",>,[[-<+>],]<."#;

    let cell_size = r#"Calculate the value 256 and test if it's zero
If the interpreter errors on overflow this is where it'll happen
++++++++[>++++++++<-]>[<++++>-]
+<[>-<
    Not zero so multiply by 256 again to get 65536
    [>++++<-]>[<++++++++>-]<[>++++++++<-]
    +>[>
        # Print "32"
        ++++++++++[>+++++<-]>+.-.[-]<
    <[-]<->] <[>>
        # Print "16"
        +++++++[>+++++++<-]>.+++++.[-]<
<<-]] >[>
    # Print "8"
    ++++++++[>+++++++<-]>.[-]<
<-]<
# Print " bit cells\n"
+++++++++++[>+++>+++++++++>+++++++++>+<<<<-]>-.>-.+++++++.+++++++++++.<.
>>.++.+++++++..<-.>>-
Clean up used cells.
[[-]<]"#;

let sier = r#"++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[-<<<[->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<]>.>+[>>]>+]"#;
let sier_result = r#"                               *
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

assert_eq!(eval_string(sier, None)?.to_string(), sier_result);
//    assert_eq!(eval_string(sum, Some(vec![1,2,3,4,5,6]))?.to_vec()[0], (0u8..=6).sum::<u8>());
    Ok(())
}

//
//
//
//
//#[test]
//fn rot13() -> Result<(), BrainfuckError> {
//    use brainfrsck::prelude::eval_string;
//    let rot13: &str = r#"
//    -,+[                         Read first character and start outer character reading loop
//        -[                       Skip forward if character is 0
//            >>++++[>++++++++<-]  Set up divisor (32) for division loop
//                                (MEMORY LAYOUT: dividend copy remainder divisor quotient zero zero)
//            <+<-[                Set up dividend (x minus 1) and enter division loop
//                >+>+>-[>>>]      Increase copy and remainder / reduce divisor / Normal case: skip forward
//                <[[>+<-]>>+>]    Special case: move remainder back to divisor and increase quotient
//                <<<<<-           Decrement dividend
//            ]                    End division loop
//        ]>>>[-]+                 End skip loop; zero former divisor and reuse space for a flag
//        >--[-[<->+++[-]]]<[         Zero that flag unless quotient was 2 or 3; zero quotient; check flag
//            ++++++++++++<[       If flag then set up divisor (13) for second division loop
//                                (MEMORY LAYOUT: zero copy dividend divisor remainder quotient zero zero)
//                >-[>+>>]         Reduce divisor; Normal case: increase remainder
//                >[+[<+>-]>+>>]   Special case: increase remainder / move it back to divisor / increase quotient
//                <<<<<-           Decrease dividend
//            ]                    End division loop
//            >>[<+>-]             Add remainder back to divisor to get a useful 13
//            >[                   Skip forward if quotient was 0
//                -[               Decrement quotient and skip forward if quotient was 1
//                    -<<[-]>>     Zero quotient and divisor if quotient was 2
//                ]<<[<<->>-]>>    Zero divisor and subtract 13 from copy if quotient was 1
//            ]<<[<<+>>-]          Zero divisor and add 13 to copy if quotient was 0
//        ]                        End outer skip loop (jump to here if ((character minus 1)/32) was not 2 or 3)
//        <[-]                     Clear remainder from first division if second division was skipped
//        <.[-]                    Output ROT13ed character from copy and clear it
//        <-,+                     Read next character
//    ]                            End character reading loop"#;
//    let result = eval_string(rot13, None)?;
//    let expected = r#""#;
////    assert_eq!(result, expected);
//    Ok(())
//}
//
//#[test]
//fn helloworld() -> Result<(), BrainfuckError> {
//    use brainfrsck::prelude::eval_string;
//    let helloworld: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
//    let result = eval_string(helloworld, None)?;
//    let expected = r#""#;
////    assert_eq!(result, expected);
//    Ok(())
//}
//
//#[test]
//fn mv() -> Result<(), BrainfuckError> {
//    use brainfrsck::prelude::eval_string;
//    let mv: &str = r#"
//        >
//        ,  dp = 1 : *dp = input1
//        [   mv 1 to dp 2
//            - subtract from 1
//            > go to 2
//            + add to 2
//            < go to 1
//        ]
//        > go to 2
//        . output
//    "#;
//    let result = eval_string(mv, None)?;
//    let expected = r#""#;
////    assert_eq!(result, expected);
//    Ok(())
//}
//
//#[test]
//fn mv2() -> Result<(), BrainfuckError> {
//    use brainfrsck::prelude::eval_string;
//    let mv2: &str = r#"
//    Code:   Pseudo code:
//    ,       Put the initial value in cell0
//    [       While cell0 is not 0
//    -       Subtract 1 from cell0
//    >>      Move the pointer to cell2
//    +       Add 1 to cell2
//    <<      Move the pointer back to cell0
//    ]      End while
//    >>.    Go to cell2 and output
//    "#;
//    let result = eval_string(mv2, None)?;
//    let expected = r#""#;
////    assert_eq!(result, expected);
//    Ok(())
//}
//
//#[test]
//fn reverse() -> Result<(), BrainfuckError> {
//    use brainfrsck::prelude::eval_string;
//    let reverse: &str = r#"
//        ,[>,]       Take all the input and put them in memory in order
//        <           Go to the last item put in memory
//        [
//            .       Output the value
//            [-]     Clear the cell
//            <       Move to the next cell down
//        ]
//    "#;
//    let result = eval_string(reverse, None)?;
//    let expected = r#""#;
////    assert_eq!(result, expected);
//    Ok(())
//}
//
//#[test]
//fn qsort() -> Result<(), BrainfuckError> {
//    use brainfrsck::prelude::eval_string;
//    let qsort: &str = r#">>+>>>>>,[>+>>,]>+[--[+<<<-]<[<+>-]<[<[->[<<<+>>>>+<-]<<[>>+>[->]<<[<]
//    <-]>]>>>+<[[-]<[>+<-]<]>[[>>>]+<<<-<[<<[<<<]>>+>[>>>]<-]<<[<<<]>[>>[>>
//    >]<+<<[<<<]>-]]+<<<]+[->>>]>>]>>[.>>>]"#;
//    let result = eval_string(qsort, None)?;
//    let expected = r#""#;
////    assert_eq!(result, expected);
//    Ok(())
//}

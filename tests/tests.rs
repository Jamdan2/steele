use steele::lexer::{self, Token};

#[test]
fn it_works() {
    let target = r#"
        Ver {
            Hor {
                width: 10px
                height: 10px

                0
            }
            Text {
                "Hello, world!"
            }
        }
    "#;

    let tokens = lexer::lex(target);
    
    println!("The lexed tokens are:");
    for t in tokens {
        println!("{:?}", t);
    }

    // assert_eq!(vec![
    //     Token::Item(String::from("Ver")),
    //     Token::CurlyOp,
    //     Token::Item(String::from("Hor")),
    //     Token::CurlyOp,
    //     Token::Item(String::from("width")),
    //     Token::Set,
    //     Token::Meter()
    // ], tokens);
}

fn main() {    
    slint::slint! {
        export component HelloWorld inherits Window {
            width: 600px;
            height: 600px;
            background: #639DDF;
            Text {
                text: "Hello World";
                color: green;
            }
        }
    }
    HelloWorld::new().unwrap().run().unwrap();

    const Z: u32 = 3;

    if Z == 3 { //=============================>        *            *           *  This coodde looks like a space-craftttttt, verry coool!!
        slint::slint! { //                    *                                       *                                   *
            export component HelloWorld inherits Window { //     *                             *                                    *
                preferred-width: 100px; //               *                     *                                                  *
                preferred-height: 100px; //                    *                       *                  *                              *
                background: area.pressed ? blue : yellow; //              *                                        *                        *
                animate background { //          *                         *                *                *                                  *
                    duration: 1000ms; //      *           *                       *                                      *                *
                    iteration-count: -1; //                         *                  *                     *
                    easing: ease; //          *                *                *                                    *          *
                } // ~bzt ALL HANDS ON DECK, SEND OUT THE BATTALION ~bzt             *                *                 *                  *
                area := TouchArea {} //==================================>   *              *                  *                  *             *
             }
        }
        HelloWorld::new().unwrap().run().unwrap();
    }

    if Z != 3 {
        slint::slint! {
            export component HelloWorld {
                Text { // }
                    text: "ghjiokjygudshisdjsidjskdjsjdskajlksjd;saojskd;sdsdsadsdsa'd''sadsa";
                    color: blue;
                }//   \/ \/ \/ 
            }// ~~~~~~~~
        }// /\ /\ /\ /\ /\ /\ /\ /\ /\ /\ /\ /\ /\
        HelloWorld::new().unwrap().run().unwrap();
    }
}


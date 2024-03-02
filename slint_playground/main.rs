fn main() {    
    slint::slint! {
        import {HorizontalBox, Button} from "std-widgets.slint";
        export component Dungeon inherits Window {
            width: 600px;
            height: 600px;
            background: @radial-gradient(circle, #FFFBC1, white, #959595, #757575, #515151, #333333, #000000);
            HorizontalBox {
                Button { text: "Fight";  }
                Button { text: "Talk"; }
                Button { text: "Flee"; }
                width: 50px;
                height: 50px;
                x: 210px;
                y: 400px;
            }
            Text {
                text: "The world turns to static, and an angel appears before you \n\n\n                                10 Hit Points left";
                color: #FF0000;
            }
        }
    }
    Dungeon::new().unwrap().run().unwrap();
}

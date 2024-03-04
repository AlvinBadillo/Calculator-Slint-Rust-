
slint::slint! {
    import { VerticalBox } from "std-widgets.slint";
    import "./abadi_mt_std_extra_light.ttf";
    import "./helvetica-light-587ebe5a59211.ttf";

    export global CLogic {
        //callback button.pressed(String);
        callback button_pressed(string);
    }
    component Button {
        in property <string> text;
        height: 100px;
        width: 100px;
        Rectangle {
            background: touch.pressed ? #d5d6d8.brighter(40%) : touch.has-hover ? #d5d6d8.brighter(10%) : #d5d6d8;
            //animate background { duration: 100ms; }
            border-width: 0.4px;
            border-color: #202020;
            touch := TouchArea{
                clicked => {CLogic.button_pressed(root.text);}
            }
        }
        Text{ 
            text :root.text; 
            font-size: 45px;
            font-family: "Abadi MT Std";
            //font-weight: ;
        }
    }
    component Button2 {
        in property <string> text;
        height: 100px;
        width: 100px;
        Rectangle {
            background: touch.pressed ? #c7c8ca.brighter(40%) : touch.has-hover ? #c7c8ca.brighter(10%) : #c7c8ca;
            //animate background { duration: 100ms; }
            border-width: 0.4px;
            border-color: #202020;
            touch := TouchArea{
                clicked => {CLogic.button_pressed(root.text);}
            }
        }
        Text{ 
            text :root.text; 
            font-size: 30px;
            font-family: "Abadi MT Std";
        }
    }
    component OrangeButton {
        in property <string> text;
        height: 100px;
        width: 100px;
        Rectangle {
            background: touch.pressed ? #f88f12.brighter(40%) : touch.has-hover ? #f88f12.brighter(10%) : #f88f12;
            //animate background { duration: 100ms; }
            border-width: 0.4px;
            border-color: #202020;
            touch := TouchArea{
                clicked => {CLogic.button_pressed(root.text);}
            }
        }
        Text{ 
            text :root.text; 
            font-size: 45px;
            font-family: "Abadi MT Std";
            color: white;
        }
    }

    export component App inherits Window {
        width: 400px;
        height: 600px;
        in property <bool> found1: false;
        in property <bool> adding_decimals: false;
        // in property <int> display: 0;
        // in property <int> value: 0;
        // in property <int> prev_value: 0;
        in property <float> display;
        in property <float> value;
        in property <float> prev_value;
        in property <string> operator: " ";
        in property <int> i: 1;
        //in property <string> prev_operator: " ";
        in property <string> empty: " ";
        //in property <int> zero: 0;
        // in property <string> value_str: "";
        // callback clicked <=> btn.clicked;

        //in property <int> desfase: 0;

        Rectangle {
                height: 100px;
                width: 400px;
                background: #202020;
                //col: 0;
                x: 0;
                y: 0;

            }
        Text{
            text: display;
            x: 3px;
            y: 10px;
            height: 150px;
            width: 400px;
            font-size: 100px;
            font-family: "Abadi MT Std";
            
            color: white;
            
        }
        GridLayout { 
            y: 100px;
            // The following are commented because they are used for texting
            //padding: 10px;
            //spacing: 10px;

            // Text{
            //     text: value;
            //     colspan: 4;
            //     //col: 3;
            //     height: 150px;
            //     width: 400px;
            //     font-size: 20px;
                
            // }
            // Text{
            //     text: prev_value;
            //     colspan: 4;
            //     //col: 3;
            //     height: 150px;
            //     width: 400px;
            //     font-size: 20px;
                
            // }
            // Text{
            //     text: prev_operator;
            //     colspan: 4;
            //     col: 3;
            //     height: 150px;
            //     width: 400px;
            //     font-size: 25px;       
            // }
            Row{
                Button2 { text: "C";}
                Button2 { text: "+/–";}
                Button2 { text: "%";}
                OrangeButton { text: "÷";}
                // OrangeButton { text: "/";}
            }
            Row{
                Button { text: "7";}
                Button { text: "8";}
                Button { text: "9";}
                OrangeButton { text: "×";}
            }
            Row{
                Button { text: "4";}
                Button { text: "5";}
                Button { text: "6";}
                OrangeButton { text: "–";}
            }
            Row{
                Button { text: "1";}
                Button { text: "2";}
                Button { text: "3";}
                OrangeButton { text: "+";}
            }
            // Row{
            //     Button { text: "0";}
            // }
        }
        GridLayout {
            y: 500px;
            Row{
                Button { text: "0"; width: 200px;}
            }
        }
        GridLayout {
            x: 200px;
            y: 500px;
            Row{
                Button { text: "."; }
                OrangeButton { text: "=";}
            }
        }
    }
}

fn main() {
    use slint::Weak;
    let app = App::new().unwrap();
    let weak = app.as_weak();

    // app.on_clicked(move || {
    //     let app = weak.upgrade().unwrap();
    //     app.set_counter(app.get_counter() + 5);
    // });
    app.global::<CLogic>().on_button_pressed(move |value| {
        let app = weak.unwrap();
        let curr_val = value.clone();       //.as_str(); //.to_string();;
        //curr_val
        if value == "." {
            app.set_adding_decimals(true);
        }
        else if value == "+/-"{
            if !app.get_found1(){
                app.set_value(app.get_value() * -1.0);
                app.set_display(app.get_value());
            }
            else{
                app.set_prev_value(app.get_prev_value() * -1.0);
                app.set_display(app.get_prev_value());
            }
        }
        else if value == "%"{
            if !app.get_found1(){
                app.set_value(app.get_value() / 100.0);
                app.set_display(app.get_value());
            }
            else{
                app.set_prev_value(app.get_prev_value() / 100.0);
                app.set_display(app.get_prev_value());
            }
        }
        else if curr_val == "C" {
            app.set_display(0.0);
            app.set_operator(app.get_empty());
            app.set_value(0.0);
            app.set_prev_value(0.0);
            app.set_adding_decimals(false);
            app.set_i(1);
            app.set_found1(false);
        }
        else if  value == "0" || value == "1" || value == "2" || value == "3" || value == "4" || value == "5" || value == "6" || value == "7" || value == "8" || value == "9" {
            //app.set_desfase(app.get_desfase() + 56);
            let val : f32 = value.parse().unwrap();  
            if !app.get_adding_decimals() { app.set_value(app.get_value() * 10.0 + val); } 
            else{ app.set_value(app.get_value() + val / 10_i32.pow(app.get_i().try_into().unwrap()) as f32); app.set_i(app.get_i() + 1); }
            
            app.set_display(app.get_value());
        }
        else{
            let copy = curr_val.clone();
            //we enter this else when we get an operator
            if !app.get_found1() {                              //if we are on the first number, swirtch values to make room for second number
                app.set_prev_value(app.get_value());
                app.set_value(0.0);
                app.set_display(app.get_prev_value());          //temporary switch so user can always see the number hes working on
                app.set_operator(value);
                app.set_found1(true);
            }
            else{                                           //we are looking for a second number, we first want to do operation, then set thing back to !found1
                if value == "+" || value == "–" || value == "÷" || value == "×" || value == "="{
                    app.set_adding_decimals(false);
                    app.set_i(1);
                    //if its one of these, we want to do operation first and then set up for next
                         if app.get_operator() == "+"  { app.set_display(app.get_value() + app.get_prev_value()); app.set_operator(value);}
                    else if app.get_operator() == "–"  { app.set_display(app.get_prev_value() - app.get_value()); app.set_operator(value);}
                    else if app.get_operator() == "×"  { app.set_display(app.get_prev_value() * app.get_value()); app.set_operator(value);}
                    else if app.get_operator() == "÷"{ app.set_display(app.get_prev_value() / app.get_value()); app.set_operator(value);} 
                    // else if app.get_operator() == " " && curr_val != "=" {     //this a secial case where we dont have a prev operator but we have a prev number
                    //          if value == "+"  { app.set_display(app.get_value() + app.get_prev_value()); app.set_operator(value);}
                    //     else if value == "-"  { app.set_display(app.get_prev_value() - app.get_value()); app.set_operator(value);}
                    //     else if value == "x"  { app.set_display(app.get_prev_value() * app.get_value()); app.set_operator(value);}
                    //     else if value == "•/•"{ app.set_display(app.get_prev_value() / app.get_value()); app.set_operator(value);}
                    // }
                    app.set_prev_value(app.get_display());      //set parameters as if we are looking for second number
                    app.set_value(0.0);

                    if copy == "=" {
                        app.set_operator(app.get_empty());
                    }
                    if copy == " " {
                        app.set_operator(copy);
                    }
                    else{
                        app.set_operator(copy);
                    }                    
                }
            }
        }
        
    });
    app.run().unwrap();
}




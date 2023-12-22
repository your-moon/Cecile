type Doughnut;

type Cruller;

impl Doughnut {
    fn new() {
    }

    fn do(s: String) {
        println "Do something with doughnut";
    }
 
    fn finish(ingredient: String) {
        println "Finish with " + ingredient;
    }

    fn cook(s: String) {
        println "Dunk in the fryer.";
        self.do("fry");
    }

}


impl Cruller < Doughnut {
    fn new() {
    }
    
    fn do(s: String) {
        println "Do something with cruller";
    }

    fn finish(ingredient: String) {
        super.finish("icing");
    }
    
    fn cook(s: String) {
        super.cook("S");
    }

}

let crul = Cruller();
crul.finish("S");
crul.cook("S");

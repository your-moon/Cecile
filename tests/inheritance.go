type Doughnut;

type Cruller;

impl Doughnut {
    fn new() {
    }

    fn finish(ingredient: String) {
        println "Finish with " + ingredient;
    }

    fn cook() {
        println "Dunk in the fryer.";
        self.finish("sprinkles");
    }

}


impl Cruller < Doughnut {
    fn new() {
    }

    fn finish(ingredient: String) {
        super.finish("icing");
    }
    
    fn cook() {
        super.cook();
    }

}

let crul = Cruller();
crul.finish("S");
crul.cook();

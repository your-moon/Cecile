type Doughnut {
}

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

type Cruller {
}

impl Cruller < Doughnut {
    fn new() {
    }

    fn finish(ingredient: String) {
        super.finish("icing");
    }

}

let crul = Cruller();
crul.finish("S");

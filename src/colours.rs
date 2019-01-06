pub mod gradient {

    #[derive(Debug, PartialEq)]
    pub struct RGBColour {
        red: i16,
        green: i16,
        blue: i16
    }

    pub fn interpolate_colour(colour1: &RGBColour, colour2: &RGBColour, factor: f64) -> RGBColour {
        let mut result = RGBColour{ red: colour1.red, green: colour1.green, blue: colour1.blue };
        result.red = result.red + (factor * (colour2.red - result.red) as f64).round() as i16;
        result.green = result.green + (factor * (colour2.green - result.green) as f64).round() as i16;
        result.blue = result.blue + (factor * (colour2.blue - result.blue) as f64).round() as i16;
        result
    }

    #[test]
    fn test_interpolate_colour() {
        let c1 = RGBColour {
            red: 1,
            green: 1,
            blue: 1
        };

        let c2 = RGBColour {
            red: 100,
            green: 200,
            blue: 255
        };

        assert_eq!(interpolate_colour(&c1, &c2, 0.5), RGBColour { red: 51, green: 101, blue: 128 });
        assert_eq!(interpolate_colour(&c1, &c2, 0.75), RGBColour { red: 75, green: 150, blue: 192 });
    }


    pub fn interpolate_colours(colour1: &RGBColour, colour2: &RGBColour, steps: i16) -> Vec<RGBColour> {
        assert!(steps > 1);
        let mut results = Vec::new();
        let step_factor = 1.0 / (steps as f64 - 1.0);

        for i in 0..steps {
            results.push(interpolate_colour(&colour1, &colour2, step_factor * i as f64));
        }

        results
    }

    #[test]
    fn test_interpolate_colours() {
        let c1 = RGBColour {
            red: 94,
            green: 79,
            blue: 162
        };

        let c2 = RGBColour {
            red: 247,
            green: 148,
            blue: 89
        };

        assert_eq!(interpolate_colours(&c1, &c2, 5),
            vec![
                RGBColour{ red:  94, green:  79, blue: 162 },
                RGBColour{ red: 132, green:  96, blue: 144 },
                RGBColour{ red: 171, green: 114, blue: 125 },
                RGBColour{ red: 209, green: 131, blue: 107 },
                RGBColour{ red: 247, green: 148, blue: 89 }
            ]
        );

    }

}

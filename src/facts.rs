use colored::Colorize;
use rand::Rng;
use serde_json::{Result, Value};

// creates a variable "data" from json that we will then parse into string
pub fn frog_fact() -> Result<()> {
    let data = r#"{
    "facts": [
                {
                    "fact": "a frog completely sheds its skin about once a week. after it pulls off the old, dead skin, the frog usually eats it."
                },
                {
                    "fact": "a group of birds is called a flock, a group of cattle is called a herd, but a group of frogs is called an army."
                },
                {
                    "fact": "there is a frog in Indonesia that has no lungs – it breathes entirely through its skin."
                },
                {
                    "fact": "frogs can meow"
                },
                {
                    "fact": "the waxy monkey frog secretes a wax from its neck and uses its legs to rub that wax all over its body. The wax prevents the skin of the frog from drying out in sunlight."
                },
                {
                    "fact": "most frogs have teeth, although usually only on their upper jaw. the teeth are used to hold prey in place until the frog can swallow it."
                },
                {
                    "fact": "the biggest frog in the world is the Goliath frog. it lives in West Africa and can measure more than a foot in length and weigh more than 7 pounds – as much as a newborn baby."
                },
                {
                    "fact": "there’s a type of poison dart frog called the blue-jeans frog; it has a red body with blue legs. it is also sometimes called the strawberry dart frog."
                },
                {
                    "fact": "the red-eyed tree frog lays it eggs on the underside of leaves that hang over water. when the eggs hatch, the tadpoles fall into the water below."
                },
                {
                    "fact": "glass frogs make their skin transparent by hiding red blood cells in their livers."
                },
                {
                    "fact": "certain species of frogs can freeze nearly solid in the winter, and emerge unscathed in the spring."
                },
                {
                    "fact": "frogs use their eyeballs to help them eat."
                },
                {
                    "fact": "the biggest frog in the world can weigh nearly seven pounds—the size of a newborn baby!"
                },
                {
                    "fact": "there are over 7,500 species of frogs on (almost) every continent on Earth"
                },
                {
                    "fact": "frogs demonstrate a range of emotions – just like humans."
                },
                {
                    "fact": "white’s tree frogs don’t need to live near water."
                },
                {
                    "fact": "some frog species can spend weeks, if not months, underwater."
                },
                {
                    "fact": "the UK has two native frog species, though several other species call our islands home."
                },
                {
                    "fact": "many frogs can jump 20 times their own height."
                },
                {
                    "fact": "frogs come in all sorts of colours."
                },
                {
                    "fact": "one of the ways you can tell a male frog from a female is by looking at their ears."
                },
                {
                    "fact": "did you know that frogs moult? This is the process where they shed their skin."
                },
                {
                    "fact": "croaking is used by male frogs as a way to attract females."
                },
                {
                    "fact": "frogs don’t drink water with their mouths; they “drink” by absorbing water through their skin."
                },
                {
                    "fact": "frogs can be found on every continent except Antarctica."
                },
                {
                    "fact": "frogs are excellent jumpers, and some can jump over 10 feet."
                },
                {
                    "fact": "frogs have ears!"
                },
                {
                    "fact": "trans rights are human rights!"
                },
                {
                    "fact": "some frogs can live to be around 12 years old."
                },
                {
                    "fact": "scientists are discovering new frogs even now!!!11!"
                },
                {
                    "fact": "the study of frogs is called Herpetology."
                },
                {
                    "fact": "frogs have 180-degree vision."
                },
                {
                    "fact": "frogs can feel happiness!"
                },
                {
                    "fact": "frogs were the first land animals with vocal cords."
                },
                {
                    "fact": "frogs have a three-chambered heart, while humans have 4."
                },
                {
                    "fact": "a frog sleeps with their eyes open."
                },
                {
                    "fact": "frog legs are a delicacy in some parts of the world."
                },
                {
                    "fact": "colorful frogs are usually poisonous to predators, so their skin is a warning system."
                },
                {
                    "fact": "frogs use their tongue to catch prey."
                },
                {
                    "fact": "a frog’s tongue is around three times longer than its body."
                },
                {
                    "fact": "the “ribbit” noise we associate with frogs is only made by the Pacific Tree Frog."
                },
                {
                    "fact": "frogs are cold-blooded and cannot regulate their own body temperature."
                }
            ]
        }
    "#;

// parses json into a string
    let v: Value = serde_json::from_str(data)?;

// chooses a random number
    let num = rand::thread_rng().gen_range(0..41);

// prints a frog fact by fetching them from json and choosing a random one
    println!("{} {}", "random frog fact:".truecolor(152,251,152).bold(), v["facts"][num]["fact"]);

    Ok(())
}
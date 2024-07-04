use nanoid::nanoid;

const ALPHABET: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'
];

pub fn generate_id() -> String {
    nanoid!(10, &ALPHABET)
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy)]
pub enum Atom<'a>{
    Num(i32),
    Str(&'a str),
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy)]
pub struct Object<'a>{
    atom: Atom<'a>,
    car: Option<u16>,
    cdr: Option<u16>
}
impl<'a> Object<'a>{
    pub fn new(atom: Atom<'a>, car: Option<u16>, cdr: Option<u16>)->Object<'a>{
        Object{atom, car, cdr}
    }
}
pub fn parse_atom<'a>(s:&'a str, mut workspace:&mut [Option<Object>;128])->Result<Atom<'a>, &'a str>{
    let s = s.trim();
    if s.eq(""){
        return Err("the atom is empty");
    }
    let result = s.parse::<i32>();
    if let Ok(i) = result {
        return Ok(Atom::Num(i));
    }
    Ok(Atom::Str(s))
}
const LIST_OPEN_DELIMITER: char = '(';
const LIST_CLOSE_DELIMITER:char = ')';
const LIST_ELEMENT_DELIMITER:char = ' ';
pub fn parse_list_actual<'a>(elements:&[u8], mut i:usize, mut workspace:&mut [Option<Object>;128])->usize{
    let mut result:Option<Object> = None;
    let mut atom_start:usize = 0;
    while i < elements.len(){
        if elements[i] == LIST_CLOSE_DELIMITER  as u8{
            workspace[i] = None;
        }
        if elements[i] == LIST_OPEN_DELIMITER as u8{
            let k = parse_list_actual(elements, i+1 , workspace);
            i = k;

        }
        if elements[i] != LIST_ELEMENT_DELIMITER as u8{}
        if elements[i] == LIST_ELEMENT_DELIMITER as u8{
            workspace[i] = Some(Object::new(Atom::Num(0), None, None));
        }
        i += 1;
    }
    i
}
pub fn parse_list<'a>(s:&str, mut workspace:&mut [Option<Object>;128])->Result<&'a str, &'a str>{
    if s.len() == 0{
        return Err("the list is empty")
    };
    let s = s.trim();
    if s.as_bytes()[0] != LIST_OPEN_DELIMITER as u8{
        return Err("the list open delimiter is not set")
    }
    let elements = s.as_bytes();
    let mut i = 0;
    parse_list_actual(elements, i, workspace);
    Ok("Ok")
}
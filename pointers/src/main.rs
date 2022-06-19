mod cons_box;
mod deref_drop;
mod rc;
mod refcell;


fn main() {
    crate::cons_box::demo();
    crate::deref_drop::demo();
    crate::rc::demo();
    crate::refcell::demo();
}

use yew::prelude::*;

pub enum SelectMode {
    None,
    Cell,
    Row,
    Column
}

pub struct Table {
    selected: Option<(u32, u32)>,
    select_mode: SelectMode
}

pub enum Msg {
    Select(u32, u32),
    UnSelect
}

impl Default for Table {
    fn default() -> Self {
        Table {
            select_mode: SelectMode::Row,
            selected: None
        }
    }
}
#[derive(Copy)]
pub enum ColumnType {

}
#[derive(Copy)]
pub struct TableColumn{
    pub name: String,
    pub col_type: ColumnType
}


pub struct TableData {

}
#[derive(Copy)]
pub struct Props {
    columns: Vec<TableColumn>,
//    data:Vec<>
}

impl Component for Table {
    type Message = Msg;
    type Properties = Props;

    fn create(props: <Self as Component>::Properties, link: ComponentLink<Self>) -> Self {
        Table::default()
    }

    fn update(&mut self, msg: <Self as Component>::Message) -> bool {
        match msg {
            Msg::Select(x, y) =>  {
                self.selected = Some((x, y));
            },
            Msg::UnSelect => {
                self.selected = None;
            }
        }
        true
    }
}

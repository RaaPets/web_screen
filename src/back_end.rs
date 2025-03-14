//use std::collections::HashMap;
//use crate::error::RunnerError;

//use task::TaskHolder;
use win_screenshot::utils::HwndName;

//  //  //  //  //  //  //  //
#[derive(Default)]
pub struct Backend {
    //counter: usize,
    //list: HashMap<usize, TaskHolder>,
}

impl Backend {
    pub fn list(&self) -> String {
        println!("backend.list()");
        let Ok(win_list) = win_screenshot::utils::window_list() else {
            return "some error".to_owned();
        };
        let mut res = String::new();
        for HwndName { hwnd, window_name }  in win_list.into_iter() {
            res += &format!("x0:{:x} <- [{}]\n", hwnd, window_name);
        }
        res
    }


    /*
    pub fn get(&self, id: usize) -> Result<String, RunnerError > {
        let Some(res) = self.list.get(&id) else {
            return Err(RunnerError::WrongId);
        };

        Ok(res.info())
    }

    pub fn insert(&mut self, info: &str) -> Result<usize, RunnerError> {
        self.counter += 1;
        let Ok(task) = TaskHolder::new(&info) else {
            todo!()
        };
        self.list.insert(self.counter, task);

        Ok(self.counter)
    }

    pub fn remove(&mut self, id: usize) -> Result<String, RunnerError> {
        let Some(res) = self.list.remove(&id) else {
            return Err(RunnerError::WrongId);
        };

        Ok(res.info())
    }

    pub fn list(&self) -> String {
        let mut text = String::new();
        for (id, item) in &self.list {
            text += &format!("id({}) <{}>\n", id, &item.info());
        }

        text
    }
    */
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
/*
#[cfg(test)]
mod runner_response_tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn deletion() -> Result<()> {
        let mut new = Runner::default();
        new.insert("one")?;
        new.insert("two")?;
        new.remove(1).unwrap();
        let id3 = new.insert("three")?;
        assert!(id3 == 3);
        let response = new.get(2)?;
        assert!(response == "two");
        let response_none = new.get(1);
        assert!(response_none.is_err());
        Ok(())
    }

    #[test]
    fn get_item() -> Result<()> {
        let mut new = Runner::default();
        new.insert("one")?;
        new.insert("two")?;
        new.insert("three")?;
        let response = new.get(2)?;
        assert!(response == "two");
        Ok(())
    }

    #[test]
    fn insertion() -> Result<()> {
        let mut new = Runner::default();
        new.insert("one")?;
        let response = new.list();
        assert!(response == "id(1) <one>\n");
        Ok(())
    }

    #[test]
    fn create_empty() {
        let new = Runner::default();
        let response = new.list();
        assert!(response == "");
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod runner_basic_tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn deletion() -> Result<()> {
        let mut new = Runner::default();
        new.insert("one")?;
        new.insert("two")?;
        assert!(new.remove(3).is_err());
        assert!(new.list.len() == 2);
        assert!(new.counter == 2);
        new.remove(2).unwrap();
        assert!(new.list.len() == 1);
        assert!(new.counter == 2);
        Ok(())
    }

    #[test]
    fn insertion() -> Result<()> {
        let mut new = Runner::default();
        let id1 = new.insert("one")?;
        assert!(id1 == 1);
        let id2 = new.insert("two")?;
        assert!(id2 == 2);
        assert!(new.list.len() == 2);
        assert!(new.counter == 2);
        Ok(())
    }

    #[test]
    fn create_empty() {
        let new = Runner::default();
        assert!(new.list.len() == 0);
        assert!(new.counter == 0);
    }
}
*/

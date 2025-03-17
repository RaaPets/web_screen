use std::io::Cursor;

use image::{DynamicImage, ImageFormat, RgbaImage};
use win_screenshot::prelude::*;
use win_screenshot::utils::HwndName;

//  //  //  //  //  //  //  //
#[derive(Default)]
pub struct Backend {}

impl Backend {
    pub fn try_list(&self) -> Result<Vec<(isize,String)>, String> {
        println!("backend.list()");
        let Ok(win_list) = win_screenshot::utils::window_list() else {
            return Err("some WinScreenshot error".to_owned());
        };
        let mut res = Vec::new();
        for HwndName { hwnd, window_name } in win_list.into_iter() {
            //res += &format!("{} x0:{:x} <- [{}]\n", hwnd, hwnd, window_name);
            res.push((hwnd, window_name));
        }
        Ok(res)
    }

    pub fn display_screenshot(&self) -> Option<Vec<u8>> {
        let Ok(buf) = capture_display() else {
            return None;
        };
        regenerator(buf)
    }
    pub fn window_screenshot(&self, hwnd: isize) -> Option<Vec<u8>> {
        let Ok(buf) = capture_window(hwnd) else {
            return None;
        };
        regenerator(buf)
    }
}

fn regenerator(buf: RgbBuf) -> Option<Vec<u8>> {
    let Some(rgba) = RgbaImage::from_raw(buf.width, buf.height, buf.pixels) else {
        return None;
    };
    let mut writter = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(rgba)
        .write_to(&mut writter, ImageFormat::Png)
        .unwrap();
    //
    let res = writter.into_inner();
    Some(res)
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

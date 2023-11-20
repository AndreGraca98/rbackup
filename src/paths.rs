use std::{
    cell::RefCell,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
    rc::Rc,
};

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

pub(crate) fn gather_paths(dir: &Path) -> Rc<RefCell<Vec<PathBuf>>> {
    let paths = Rc::new(RefCell::new(Vec::new()));
    let paths_clone = paths.clone();
    visit_dirs(dir, &|entry| {
        paths_clone.borrow_mut().push(entry.path());
    })
    .expect("Failed to visit directories");
    paths
}

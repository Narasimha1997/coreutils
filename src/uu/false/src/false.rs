//  * This file is part of the uutils coreutils package.
//  *
//  * (c) Jordi Boggiano <j.boggiano@seld.be>
//  *
//  * For the full copyright and license information, please view the LICENSE
//  * file that was distributed with this source code.

use clap::App;
use uucore::error::UResult;

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    uu_app().get_matches_from(args);
    Err(1.into())
}

pub fn uu_app<'a>() -> App<'a> {
    App::new(uucore::util_name())
}

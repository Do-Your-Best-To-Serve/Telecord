import { launch} from "@astral/astral";

const browser = await launch({ headless: false });

const page = await browser.newPage("https://x.com/i/flow/login");

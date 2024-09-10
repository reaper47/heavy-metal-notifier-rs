use maud::{html, Markup, DOCTYPE};

use crate::web::templates::Page;

pub fn layout(title: &str, is_show_nav: bool, page: Page, content: Markup) -> Markup {
    html!(
        (DOCTYPE)
        html lang="en" {
            (head(title))
            @if is_show_nav {
                (nav(page))
            }
            body class="h-screen font-sans anti-aliased" {
                main class="grid h-full w-full md:grid-cols-12" {
                    (content)
                    @if is_show_nav {
                        (footer())
                    }
                }
            }
            script defer src="/static/js/core.min.js" {}
        }
    )
}

fn head(title: &str) -> Markup {
    html!(
        head {
            title {
                @if title.is_empty() {
                    "Heavy Metal Releases"
                } @else {
                    (title) " | Heavy Metal Releases"
                }
            }
            meta charset="UTF-8";
            meta http-equiv="X-UA-Compatible" content="IE=edge";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            meta name="description" content="Be notified of new heavy metal album releases.";
            meta name="keywords" content="heavy metal, album releases, automation";
            link rel="canonical" href="https://metal.musicavis.ca/";
            link rel="icon" href="/static/favicon.png" type="image/x-icon";
            link rel="stylesheet" href="/static/css/tailwind.css";
        }
    )
}

fn nav(page: Page) -> Markup {
    html!(
        nav {
            div class="container mx-auto px-6 py-2 flex justify-between items-center" {
                a class="font-bold lg:text-3xl" style="display: ruby" href="/" {
                    img src="/static/img/logo-64x64.png" alt="logo" class="w-[2.5rem]" {
                        "Heavy Metal Releases"
                    }
                }
                div class="flex flex-col relative" {
                    div id="menu-icon" class="block lg:hidden" {
                        button class="flex items-center px-3 py-2 border rounded text-gray-500 border-gray-600 hover:text-gray-800 hover:border-teal-500 appearance-none focus:outline-none" {
                            svg class="fill-current h-3 w-3" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" {
                                title {
                                 "Menu"
                                }
                                path d="M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z" {}
                            }
                        }
                    }
                    div id="menu" class="hidden absolute top-[2.45rem]  bg-white right-[-1.5rem] w-[100vw] text-center lg:block lg:relative lg:w-auto lg:bg-transparent lg:top-0" {
                        ul class="lg:flex" {
                            li class="py-2" {
                                a class={
                                    "px-4"
                                    @if page == Page::Home { " font-bold"}
                                    @if page != Page::Home { " hover:text-gray-800" }
                                }
                                href="/" {
                                    "Home"
                                }
                            }
                            li class="py-2" {
                                a class={
                                    "px-4"
                                    @if page == Page::About { " font-bold"}
                                    @if page != Page::Home { " hover:text-gray-800" }
                                } href="/about" {
                                    "About"
                                }
                            }
                            li class="py-2" {
                                a class={
                                    "px-4"
                                    @if page == Page::Contact { " font-bold"}
                                    @if page != Page::Home { " hover:text-gray-800" }
                                } href="/contact" {
                                    "Contact"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

fn footer() -> Markup {
    html!(
        footer class="col-span-12 bg-gray-100" {
            div class="container mx-auto pt-10 pb-6" {
                div class="flex flex-wrap" {
                    div class="w-full md:w-1/3 text-center md:text-center" {
                        h5 class="uppercase mb-6 font-bold"{
                            "Links"
                        }
                        ul class="mb-4" {
                            li class="mt-2" {
                                a href="/contact" class="hover:underline text-gray-600 hover:text-orange-500" {
                                    "Support"
                                }
                            }
                            li class="mt-2" {
                                a href="/sitemap" class="hover:underline text-gray-600 hover:text-orange-500" {
                                    "Sitemap"
                                }
                            }
                        }
                    }
                    div class="w-full md:w-1/3 text-center md:text-center" {
                        h5 class="uppercase mb-6 font-bold" {
                            "Legal"
                        }
                        ul class="mb-4" {
                            li class="mt-2" {
                                a href="/tos" class="hover:underline text-gray-600 hover:text-orange-500" {
                                    "Terms of Service"
                                }
                            }
                            li class="mt-2" {
                                a href="/privacy" class="hover:underline text-gray-600 hover:text-orange-500" {
                                    "Privacy Policy"
                                }
                            }
                        }
                    }
                    div class="w-full md:w-1/3 text-center md:text-center" {
                        h5 class="uppercase mb-6 font-bold" {
                            "Service"
                        }
                        ul class="mb-4" {
                            li class="mt-2" {
                                a href="/about" class="hover:underline text-gray-600 hover:text-orange-500" {
                                    "About Us"
                                }
                            }
                            li class="mt-2" {
                                a href="/contact" class="hover:underline text-gray-600 hover:text-orange-500" {
                                    "Contact"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

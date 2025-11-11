use std::time::Duration;

use biji_ui::components::dialog::{self as dialogui, context::DialogContext};
use leptos::{portal::Portal, prelude::*};
use leptos_router::{components::Outlet, hooks::use_location};
use leptos_use::use_media_query;

use crate::components::{
    icons,
    language::LanguageMenu,
    theme::{ActiveThemeIcon, ThemeMode},
};
use crate::i18n::*;

#[component]
pub fn EmptyScaffold() -> impl IntoView {
    view! { <Outlet /> }
}

#[component]
pub fn DocsScaffold() -> impl IntoView {
    view! {
        <div class="h-full lg:ml-72 xl:ml-80">
            <header class="contents lg:flex lg:fixed lg:inset-0 lg:z-40 lg:pointer-events-none">
                <div class="contents lg:block lg:overflow-y-auto lg:px-6 lg:pt-4 lg:pb-8 lg:w-72 lg:border-r lg:pointer-events-auto xl:w-80 lg:border-zinc-900/10 lg:dark:border-white/10">
                    <div class="hidden lg:flex">
                        <a class="text-black dark:text-white" aria-label="Home" href="/">
                            "Borang"
                        </a>
                    </div>
                    <TopNav />
                    <SidebarNav class="hidden lg:block lg:mt-10" />
                </div>
            </header>
            <main class="docs mt-15">
                <Outlet />
            </main>
        </div>
    }
}

#[component]
pub fn TopNav() -> impl IntoView {
    view! {
        <div
            style="--bg-opacity-light: 0.5; --bg-opacity-dark: 0.2; --scrollbar-width-nav: var(--scrollbar-width, 0px);"
            class="fixed inset-x-0 top-0 z-40 flex h-14 items-center justify-between gap-12 pl-4 pr-[calc(var(--scrollbar-width-nav)+1rem)] transition sm:pl-6 sm:pr-[calc(var(--scrollbar-width-nav)+1.5rem)] lg:left-72 lg:z-20 lg:pl-8 lg:pr-[calc(var(--scrollbar-width-nav)+2rem)] xl:left-80 backdrop-blur-sm lg:left-72 xl:left-80 dark:backdrop-blur bg-white/[var(--bg-opacity-light)] dark:bg-zinc-900/[var(--bg-opacity-dark)]"
        >
            <div class="absolute inset-x-0 top-full h-px transition bg-border"></div>
            <div class="hidden lg:block lg:flex-auto lg:max-w-md"></div>
            <div class="flex gap-5 items-center lg:hidden">
                <Sidebar />
                <a aria-label="Home" href="/">
                    "Borang"
                </a>
            </div>
            <div class="flex gap-5 items-center">
                <nav class="hidden md:block">
                    <ul role="list" class="flex gap-5 items-center">
                        <li>
                            <a
                                class="text-sm leading-5 transition text-foreground-600 hover:text-foreground-900"
                                href="https://docs.rs/borang/latest/borang/"
                                title="Documentation"
                            >
                                <icons::BookText class="w-5 h-5" />
                            </a>
                        </li>
                        <li>
                            <a
                                class="text-sm leading-5 transition text-foreground-600 hover:text-foreground-900"
                                href="https://github.com/jonsaw/borang"
                                title="Github"
                            >
                                <icons::Github class="w-5 h-5" />
                            </a>
                        </li>
                        <li>
                            <a
                                class="text-sm leading-5 transition text-foreground-600 hover:text-foreground-900"
                                href="https://github.com/jonsaw/borang/issues"
                                title="Report an issue"
                            >
                                <icons::Bug class="w-5 h-5" />
                            </a>
                        </li>
                    </ul>
                </nav>
                <div class="hidden md:block md:w-px md:h-5 md:bg-border"></div>
                <div class="flex gap-4">
                    <LanguageMenu>
                        <icons::Language class="w-5 h-5" />
                    </LanguageMenu>
                    <ThemeMode>
                        <ActiveThemeIcon />
                    </ThemeMode>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SidebarTrigger() -> impl IntoView {
    let ctx = expect_context::<DialogContext>();

    let is_large_screen = use_media_query("(min-width: 1024px)");

    Effect::new(move |_| {
        if is_large_screen.get() {
            ctx.close();
        }
    });

    view! {
        <Show
            when=move || !ctx.open.get()
            fallback=|| {
                view! { <icons::X class="w-5 text-foreground"></icons::X> }
            }
        >

            <icons::AlignJustify class="w-5 text-foreground"></icons::AlignJustify>
        </Show>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <dialogui::Root hide_delay=Duration::from_millis(300)>
            <dialogui::Trigger class="flex justify-center items-center w-6 h-6 rounded-md transition dark:hover:bg-white/5 hover:bg-zinc-900/5">
                <SidebarTrigger />
            </dialogui::Trigger>
            <Portal>
                <dialogui::Overlay
                    class="fixed inset-0 top-14 z-30 transition-opacity duration-300 ease-linear bg-zinc-400/20 backdrop-blur-sm dark:bg-black/40"
                    show_class="opacity-100"
                    hide_class="opacity-0"
                ></dialogui::Overlay>
                <dialogui::Content
                    class="overflow-y-auto fixed bottom-0 left-0 top-14 z-30 px-4 pt-6 pb-4 w-full bg-white ring-1 shadow-lg transition duration-300 ease-in-out sm:px-6 sm:pb-10 shadow-zinc-900/10 ring-zinc-900/10 min-[416px]:max-w-sm dark:bg-zinc-900 dark:ring-zinc-800"
                    show_class="translate-x-0"
                    hide_class="-translate-x-full"
                >
                    <div>
                        <SidebarNav />
                    </div>
                </dialogui::Content>
            </Portal>
        </dialogui::Root>
    }
}

#[component]
pub fn SidebarNav(#[prop(into, optional)] class: String) -> impl IntoView {
    let i18n = use_i18n();
    let location = use_location();

    let introduction = [("/docs/intro", move || t_string!(i18n, intro.title))];

    let dialog_ctx = use_context::<DialogContext>();

    view! {
        <nav class=class>
            <ul role="list">
                <li class="md:hidden">
                    <a
                        class="block flex py-1 text-sm transition text-foreground-600 hover:text-foreground-900"
                        href="https://docs.rs/pane-resizer/latest/pane_resizer/"
                    >
                        "Documentation"
                    </a>
                </li>
                <li class="md:hidden">
                    <a
                        class="block flex py-1 text-sm transition text-foreground-600 hover:text-foreground-900"
                        href="https://github.com/jonsaw/pane-resizer"
                    >
                        "Github"
                    </a>
                </li>
                <li class="md:hidden">
                    <a
                        class="block flex py-1 text-sm transition text-foreground-600 hover:text-foreground-900"
                        href="https://github.com/jonsaw/pane-resizer/issues"
                    >
                        "Report an issue"
                    </a>
                </li>
            </ul>
            <ul role="list">
                <li class="relative mt-6 md:mt-0">
                    <h2 class="text-xs font-semibold dark:text-white text-zinc-900">
                        "Introduction"
                    </h2>
                    <ul class="border-l border-transparent">
                        {introduction
                            .into_iter()
                            .map(|(path, title)| {
                                view! {
                                    <li class="relative">
                                        <a
                                            href=path
                                            class="flex gap-2 justify-between py-1 pr-3 pl-4 text-sm transition text-foreground-600 hover:text-foreground-900"
                                            style:font-weight=move || {
                                                if location.pathname.get() == path { "500" } else { "400" }
                                            }

                                            on:click=move |_| {
                                                if let Some(ctx) = dialog_ctx {
                                                    ctx.close();
                                                }
                                            }
                                        >
                                            {title}
                                        </a>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </li>
            </ul>
        </nav>
    }
}

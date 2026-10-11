# SPDX-License-Identifier: MIT
"""The floating shelf stages file references without moving sources."""


def _shelf(strata):
    return strata.wait(
        lambda: strata.application.application_node.find(role="frame", name="Strata Shelf"),
        "the floating shelf to appear",
    )


def _open_shelf(strata):
    strata.keyboard.press("ctrl+shift+space")
    return _shelf(strata)


def _drag_onto_shelf(strata, entry, shelf):
    # Without a window manager the shelf opens over the row icons, so start on the size column.
    size = entry.find_all(role="label")[-1]
    strata.pointer.drag_points(size.screen_bounds().center, shelf.screen_bounds().center)


def _count(strata, count):
    return strata.wait(
        lambda: _shelf(strata).find(name=f"{count} item{'s' if count != 1 else ''}"),
        f"shelf to hold {count} items",
    )


def _drag_stack(strata, shelf, destination, *, modifiers=()):
    preview = shelf.find(role="grouping", name="Shelf preview")
    assert preview is not None, shelf.dump()
    strata.pointer.drag_points(
        preview.screen_bounds().center,
        destination.screen_bounds().center,
        modifiers=modifiers,
    )


def test_collect_from_two_folders_and_copy_out(strata):
    fixture = strata.fixture
    shelf = _open_shelf(strata)
    _drag_onto_shelf(strata, strata.entry("todo.txt"), shelf)
    _count(strata, 1)
    assert fixture.path("todo.txt").read_text() == "todo\n"

    strata.open_directory("documents")
    _drag_onto_shelf(strata, strata.entry("notes.txt", directory="documents"), shelf)
    _count(strata, 2)
    assert fixture.path("documents/notes.txt").exists()

    _drag_stack(strata, shelf, strata.entry("archive"), modifiers=["ctrl"])
    strata.wait(
        lambda: fixture.path("archive/todo.txt").exists()
        and fixture.path("archive/notes.txt").exists(),
        "both collected files to copy into archive",
    )
    assert fixture.path("todo.txt").exists()
    assert fixture.path("documents/notes.txt").exists()
    _count(strata, 2)


def test_shelf_is_shared_across_tabs(strata):
    fixture = strata.fixture
    shelf = _open_shelf(strata)
    _drag_onto_shelf(strata, strata.entry("todo.txt"), shelf)
    _count(strata, 1)

    strata.pointer.click(strata.window.find(role="button", name="New tab"))
    _drag_stack(strata, shelf, strata.entry("archive"), modifiers=["ctrl"])
    strata.wait(lambda: fixture.path("archive/todo.txt").exists(), "copy from the other tab")
    assert fixture.path("todo.txt").exists()


def test_drag_shelved_stack_into_folder(strata):
    fixture = strata.fixture
    shelf = _open_shelf(strata)
    _drag_onto_shelf(strata, strata.entry("todo.txt"), shelf)
    _count(strata, 1)
    _drag_stack(strata, shelf, strata.entry("archive"))
    strata.wait(lambda: fixture.path("archive/todo.txt").exists(), "shelved file to arrive")
    assert not fixture.path("todo.txt").exists()
    _count(strata, 0)

const confirmModal = document.getElementById(
    "confirm-modal",
) as HTMLDialogElement;

confirmModal.querySelector(".btn-primary")

function askConferm(id: string) {
    _getModal(id).showModal();
}

function denyConferm(id: string) {
    _getModal(id).close();
}

function hasConferm(id: string): boolean {
    return _getModal(id).open;
}

function _getModal(id: string) {
    const modal = document.getElementById(id) as HTMLDialogElement | undefined;
    if (!modal) throw new Error(`Modal ${id} not found`);
    return modal;
}

export default { askConferm, denyConferm, exeAction };

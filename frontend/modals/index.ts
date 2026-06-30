function open(id: string) {
    _getModal(id).showModal();
}

function close(id: string) {
    _getModal(id).close();
}

async function exeAction<T>(id: string, action: () => T | Promise<T>) {
    const modal = _getModal(id);
    await action();
    modal.close();
}

function _getModal(id: string) {
    const modal = document.getElementById(id) as HTMLDialogElement | undefined;
    if (!modal) throw new Error(`Modal ${id} not found`);
    return modal;
}

export default { open, close, exeAction };

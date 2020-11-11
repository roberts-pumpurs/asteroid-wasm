export interface Item {
    pk?: number,
}

export interface Mouse extends Item {
    model: string,
    manufacturer: string,
    dpi: number,
    buttons: number,
    wireless: boolean,
}

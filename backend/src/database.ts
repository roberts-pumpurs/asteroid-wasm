import { Item, Mouse } from './models';

interface TableFollower<T extends Item> {
  maxPk: number,
  items: Array<T>;
}

class FakeTable<T extends Item> {
  items: TableFollower<T>;

  constructor() {
    this.items = {
      maxPk: 0,
      items: [],
    };
  }

  getAll(): Array<T> {
    return this.items.items;
  }

  get(pk: number): T | null {
    const resultList = this.items.items.filter((el) => el.pk === pk);
    if (resultList.length > 0) {
      return resultList[0];
    }
    return null;
  }

  create(item: T): boolean {
    if ('pk' in item) {
      return false;
    }
    this.items.items.push({ ...item, pk: this.items.maxPk + 1 });
    this.items.maxPk += 1;
    return true;
  }

  update(item: T): boolean {
    // If no PK attached or the item does not exist
    if (!('pk' in item) || item.pk === undefined) {
      return false;
    }
    // hard replace the item with matching PK
    this.items.items = this.items.items.map(
      (el) => (el.pk === item.pk ? item : el),
    );
    return true;
  }

  delete(pk: number): boolean {
    const lengthBefore = this.items.items.length;
    this.items.items = this.items.items.filter((el) => el.pk !== pk);
    return lengthBefore > this.items.items.length;
  }
}
const mouseDatabaseTable = new FakeTable<Mouse>();

// Static data insertion
mouseDatabaseTable.create({
  model: 'EC2-A',
  manufacturer: 'Zowie',
  dpi: 800,
  buttons: 5,
  wireless: false,
});
mouseDatabaseTable.create({
  model: 'Viper Ultralight',
  manufacturer: 'Razer',
  dpi: 16000,
  buttons: 8,
  wireless: false,
});
mouseDatabaseTable.create({
  model: 'G903',
  manufacturer: 'Logitech',
  dpi: 16000,
  buttons: 11,
  wireless: true,
});

export default { mouse: mouseDatabaseTable };

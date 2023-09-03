use std::fmt::{self, Display};

const AGED_BRIE: &str = "Aged Brie";
const BACKSTAGE_PASSES: &str = "Backstage passes to a TAFKAL80ETC concert";
const SULFURAS: &str = "Sulfuras, Hand of Ragnaros";
const CONJURED: &str = "Conjured Mana Cake";

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            let item = &mut self.items[i];
            let updater = get_item_updater(item);
            updater.update(item);
        }
    }
}

trait ItemUpdater {
    fn update(&self, item: &mut Item) {
        self.update_sell_in(item);
        self.update_quality(item);
    }
    fn update_sell_in(&self, item: &mut Item) {
        decrease_sell_in(item);
    }
    fn update_quality(&self, item: &mut Item);
}

struct GenericItemUpdater;
impl ItemUpdater for GenericItemUpdater {
    fn update_quality(&self, item: &mut Item) {
        decrease_quality(item, 1);
        if item.sell_in <= 0 {
            decrease_quality(item, 1);
        }
    }
}

struct AgedBrieUpdater;
impl ItemUpdater for AgedBrieUpdater {
    fn update_quality(&self, item: &mut Item) {
        increase_quality(item, 1);
        if item.sell_in < 0 {
            increase_quality(item, 1);
        }
    }
}

struct BackstagePassesUpdater;
impl ItemUpdater for BackstagePassesUpdater {
    fn update_quality(&self, item: &mut Item) {
        increase_quality(item, 1);
        if item.sell_in < 10 {
            increase_quality(item, 1);
        }
        if item.sell_in < 5 {
            increase_quality(item, 1);
        }
        if item.sell_in < 0 {
            item.quality = 0;
        }
    }
}

struct SulfurasUpdater;
impl ItemUpdater for SulfurasUpdater {
    fn update_sell_in(&self, _item: &mut Item) {}
    fn update_quality(&self, _item: &mut Item) {}
}

fn get_item_updater(item: &Item) -> Box<dyn ItemUpdater> {
    match &item.name as &str {
        AGED_BRIE => Box::new(AgedBrieUpdater),
        BACKSTAGE_PASSES => Box::new(BackstagePassesUpdater),
        SULFURAS => Box::new(SulfurasUpdater),
        CONJURED => Box::new(GenericItemUpdater),
        _ => Box::new(GenericItemUpdater),
    }
}

fn decrease_sell_in(item: &mut Item) {
    item.sell_in -= 1;
}

fn decrease_quality(item: &mut Item, amount: u32) {
    item.quality = (item.quality - amount as i32).max(0);
}

fn increase_quality(item: &mut Item, amount: u32) {
    item.quality = (item.quality + amount as i32).min(50);
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{GildedRose, Item};

    const FOO: &str = "foo";
    const AGED_BRIE: &str = "Aged Brie";
    const BACKSTAGE_PASSES: &str = "Backstage passes to a TAFKAL80ETC concert";
    const SULFURAS: &str = "Sulfuras, Hand of Ragnaros";
    const CONJURED: &str = "Conjured Mana Cake";

    #[test]
    pub fn generic_item_decrease_sell_in() {
        let items = vec![Item::new(FOO, 1, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(0, rose.items[0].sell_in);
    }

    #[test]
    pub fn generic_item_decrease_quality() {
        let items = vec![Item::new(FOO, 1, 1)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(0, rose.items[0].quality);
    }

    #[test]
    pub fn generic_item_decrease_quality_twice_as_fast() {
        let items = vec![Item::new(FOO, 0, 2)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(0, rose.items[0].quality);
    }

    #[test]
    pub fn generic_item_quality_never_negative() {
        let items = vec![Item::new(FOO, 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(0, rose.items[0].quality);
    }

    #[test]
    pub fn aged_brie_increase_quality() {
        let items = vec![Item::new(AGED_BRIE, 1, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(1, rose.items[0].quality);
    }

    #[test]
    pub fn aged_brie_increase_quality_twice_as_fast() {
        let items = vec![Item::new(AGED_BRIE, 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(2, rose.items[0].quality);
    }

    #[test]
    pub fn aged_brie_quality_never_more_than_50() {
        let items = vec![Item::new(AGED_BRIE, 0, 50)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(50, rose.items[0].quality);
    }

    #[test]
    pub fn sulfuras_never_decrease_sell_in() {
        let items = vec![Item::new(SULFURAS, 0, 80)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(0, rose.items[0].sell_in);
    }

    #[test]
    pub fn sulfuras_never_decrease_quality() {
        let items = vec![Item::new(SULFURAS, 0, 80)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(80, rose.items[0].quality);
    }

    #[test]
    pub fn backstage_passes_increase_quality() {
        let items = vec![Item::new(BACKSTAGE_PASSES, 11, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(1, rose.items[0].quality);
    }

    #[test]
    pub fn backstage_passes_increase_quality_twice_as_fast() {
        let items = vec![Item::new(BACKSTAGE_PASSES, 10, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(2, rose.items[0].quality);
    }

    #[test]
    pub fn backstage_passes_increase_quality_thrice_as_fast() {
        let items = vec![Item::new(BACKSTAGE_PASSES, 5, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(3, rose.items[0].quality);
    }

    #[test]
    pub fn backstage_passes_quality_never_more_than_50() {
        let items = vec![Item::new(BACKSTAGE_PASSES, 5, 50)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(50, rose.items[0].quality);
    }

    #[test]
    pub fn backstage_passes_quality_drops_to_zero_after_concert() {
        let items = vec![Item::new(BACKSTAGE_PASSES, 0, 50)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(0, rose.items[0].quality);
    }
}

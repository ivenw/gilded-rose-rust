use std::fmt::{self, Display};
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
            if self.items[i].name != "Aged Brie"
                && self.items[i].name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if self.items[i].quality > 0 {
                    if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                        self.items[i].quality = self.items[i].quality - 1;
                    }
                }
            } else {
                if self.items[i].quality < 50 {
                    self.items[i].quality = self.items[i].quality + 1;

                    if self.items[i].name == "Backstage passes to a TAFKAL80ETC concert" {
                        if self.items[i].sell_in < 11 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }

                        if self.items[i].sell_in < 6 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }
                    }
                }
            }

            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                self.items[i].sell_in = self.items[i].sell_in - 1;
            }

            if self.items[i].sell_in < 0 {
                if self.items[i].name != "Aged Brie" {
                    if self.items[i].name != "Backstage passes to a TAFKAL80ETC concert" {
                        if self.items[i].quality > 0 {
                            if self.items[i].name != "Sulfuras, Hand of Ragnaros" {
                                self.items[i].quality = self.items[i].quality - 1;
                            }
                        }
                    } else {
                        self.items[i].quality = self.items[i].quality - self.items[i].quality;
                    }
                } else {
                    if self.items[i].quality < 50 {
                        self.items[i].quality = self.items[i].quality + 1;
                    }
                }
            }
        }
    }
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

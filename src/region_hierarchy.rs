use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::json;

#[derive(Default, Clone, Debug)]
struct RegionTreeElement {
    offers: Vec<u32>,
    sub_regions: Option<Vec<u8>>,
}

#[derive(Default, Debug, Clone)]
pub struct RegionTree {
    regions: Vec<RegionTreeElement>,
}

impl RegionTree {
    pub fn populate_with_regions(root: &Region) -> RegionTree {
        let mut tree = RegionTree::default();
        tree.regions = vec![RegionTreeElement::default(); 125];
        tree.populate_with_regions_recursive(root);
        tree
    }

    fn populate_with_regions_recursive(&mut self, region: &Region) {
        for subregion in &region.subregions {
            self.regions[region.id as usize]
                .sub_regions
                .get_or_insert_with(|| Vec::new())
                .push(subregion.id);
            self.populate_with_regions_recursive(subregion);
        }
    }

  pub fn get_available_offers(&self, region_id: u8) -> impl Iterator<Item = u32> + '_ {
    self.get_available_offers_recursive(region_id)
  }

  pub fn clear_offers(&mut self) {
    for element in &mut self.regions {
      element.offers.clear();
    }
  }

  fn get_available_offers_recursive(
    &self,
    region_id: u8,
  ) -> Box<dyn Iterator<Item = u32> + '_> {
    let current_offers = self.regions[region_id as usize]
        .offers
        .iter()
        .copied();

    let sub_region_offers = self.regions[region_id as usize]
        .sub_regions
        .iter()
        .flatten()
        .flat_map(move |&sub_region_id| self.get_available_offers_recursive(sub_region_id));

    Box::new(current_offers.chain(sub_region_offers))
  }


  pub fn insert_offer(&mut self, region_id: u8, offer_idx: u32) {
        self.regions[region_id as usize].offers.push(offer_idx);
    }

    pub fn insert_offers(&mut self, region_id: u8, offer_idxs: impl IntoIterator<Item = u32>) {
        self.regions[region_id as usize].offers.extend(offer_idxs);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_should_work() {
        let root = super::ROOT_REGION.clone();
        let mut tree = super::RegionTree::populate_with_regions(&root);
        assert_eq!(tree.get_available_offers(0).collect::<Vec<_>>(), Vec::<u32>::new());

        tree.insert_offer(0, 1);
        tree.insert_offer(1, 2);
        tree.insert_offer(2, 3);
        tree.insert_offer(3, 4);
        tree.insert_offer(4, 5);

        assert_eq!(tree.get_available_offers(0).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
        assert_eq!(tree.get_available_offers(1).collect::<Vec<_>>(), vec![2]);
        assert_eq!(tree.get_available_offers(2).collect::<Vec<_>>(), vec![3]);
        assert_eq!(tree.get_available_offers(3).collect::<Vec<_>>(), vec![4]);
        assert_eq!(tree.get_available_offers(4).collect::<Vec<_>>(), vec![5]);
        assert_eq!(tree.get_available_offers(5).collect::<Vec<_>>(), Vec::<u32>::new());
    }
}

#[derive(Deserialize, Clone)]
pub struct Region {
    id: u8,
    subregions: Vec<Region>,
}

pub static ROOT_REGION: Lazy<Region> = Lazy::new(|| {
    serde_json::from_value(json!(
    {
      "id": 0,
      "name": "European Union",
      "subregions": [
        {
          "id": 1,
          "name": "Germany",
          "subregions": [
            {
              "id": 7,
              "name": "Berlin",
              "subregions": [
                {
                  "id": 21,
                  "name": "Mitte",
                  "subregions": [
                    {
                      "id": 58,
                      "name": "Brandenburg Gate",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 59,
                      "name": "Berlin Cathedral",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 22,
                  "name": "Kreuzberg",
                  "subregions": [
                    {
                      "id": 60,
                      "name": "East Side Gallery",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 61,
                      "name": "Checkpoint Charlie",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 23,
                  "name": "Berlin Brandenburg Airport",
                  "subregions": [
                    {
                      "id": 62,
                      "name": "Terminal A",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 63,
                      "name": "Terminal B",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            },
            {
              "id": 8,
              "name": "Munich",
              "subregions": [
                {
                  "id": 24,
                  "name": "Maxvorstadt",
                  "subregions": [
                    {
                      "id": 64,
                      "name": "Oper Munich",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 65,
                      "name": "University of Munich",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 25,
                  "name": "Neuhausen Nymphenburg",
                  "subregions": [
                    {
                      "id": 66,
                      "name": "Nymphenburg Palace",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 67,
                      "name": "CHECK24 Office",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 26,
                  "name": "Schwabing",
                  "subregions": [
                    {
                      "id": 68,
                      "name": "English Garden",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 69,
                      "name": "Augustiner Brewery",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 27,
                  "name": "Old Town",
                  "subregions": [
                    {
                      "id": 70,
                      "name": "Viktualienmarkt",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 71,
                      "name": "Marienplatz",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 28,
                  "name": "Munich Airport",
                  "subregions": [
                    {
                      "id": 72,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 73,
                      "name": "Terminal 2",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            },
            {
              "id": 9,
              "name": "Frankfurt",
              "subregions": [
                {
                  "id": 29,
                  "name": "Sachsenhausen",
                  "subregions": [
                    {
                      "id": 74,
                      "name": "Eiserner Steg",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 75,
                      "name": "Museum Embankment",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 30,
                  "name": "Frankfurt Airport",
                  "subregions": [
                    {
                      "id": 76,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 77,
                      "name": "Terminal 2",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        {
          "id": 2,
          "name": "France",
          "subregions": [
            {
              "id": 10,
              "name": "Paris",
              "subregions": [
                {
                  "id": 31,
                  "name": "Charles de Gaulle Airport",
                  "subregions": [
                    {
                      "id": 78,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 79,
                      "name": "Terminal 2A",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 80,
                      "name": "Terminal 2B",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 81,
                      "name": "Terminal 2C",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 32,
                  "name": "Orly Airport",
                  "subregions": [
                    {
                      "id": 82,
                      "name": "Terminal South",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 83,
                      "name": "Terminal West",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 33,
                  "name": "1st Arrondissement",
                  "subregions": [
                    {
                      "id": 84,
                      "name": "Louvre",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 85,
                      "name": "Palais Royal",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 34,
                  "name": "7th Arrondissement",
                  "subregions": [
                    {
                      "id": 86,
                      "name": "Eiffel Tower",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 87,
                      "name": "Champ de Mars",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 35,
                  "name": "Montmartre",
                  "subregions": [
                    {
                      "id": 88,
                      "name": "Sacré-Cœur Basilica",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 89,
                      "name": "Place du Tertre",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            },
            {
              "id": 11,
              "name": "Nice",
              "subregions": [
                {
                  "id": 36,
                  "name": "Nice Côte d'Azur Airport",
                  "subregions": [
                    {
                      "id": 90,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 91,
                      "name": "Terminal 2",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 37,
                  "name": "Old Town",
                  "subregions": [
                    {
                      "id": 92,
                      "name": "Promenade des Anglais",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 93,
                      "name": "Castle Hill",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        {
          "id": 3,
          "name": "Italy",
          "subregions": [
            {
              "id": 12,
              "name": "Rome",
              "subregions": [
                {
                  "id": 38,
                  "name": "Leonardo da Vinci–Fiumicino Airport",
                  "subregions": [
                    {
                      "id": 94,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 95,
                      "name": "Terminal 3",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 39,
                  "name": "Centro Storico",
                  "subregions": [
                    {
                      "id": 96,
                      "name": "Colosseum",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 97,
                      "name": "Pantheon",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            },
            {
              "id": 13,
              "name": "Milan",
              "subregions": [
                {
                  "id": 40,
                  "name": "Malpensa Airport",
                  "subregions": [
                    {
                      "id": 98,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 99,
                      "name": "Terminal 2",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 41,
                  "name": "Linate Airport",
                  "subregions": [
                    {
                      "id": 100,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 42,
                  "name": "Brera",
                  "subregions": [
                    {
                      "id": 101,
                      "name": "Pinacoteca di Brera",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 102,
                      "name": "Brera Botanical Garden",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            },
            {
              "id": 14,
              "name": "Venice",
              "subregions": [
                {
                  "id": 43,
                  "name": "Venice Marco Polo Airport",
                  "subregions": [
                    {
                      "id": 103,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 44,
                  "name": "San Marco",
                  "subregions": [
                    {
                      "id": 104,
                      "name": "St. Mark's Basilica",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 105,
                      "name": "Doge's Palace",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        {
          "id": 4,
          "name": "Portugal",
          "subregions": [
            {
              "id": 15,
              "name": "Lisbon",
              "subregions": [
                {
                  "id": 45,
                  "name": "Lisbon Airport",
                  "subregions": [
                    {
                      "id": 106,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 107,
                      "name": "Terminal 2",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 46,
                  "name": "Alfama",
                  "subregions": [
                    {
                      "id": 108,
                      "name": "São Jorge Castle",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 109,
                      "name": "Fado Museum",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            },
            {
              "id": 16,
              "name": "Porto",
              "subregions": [
                {
                  "id": 47,
                  "name": "Porto Airport",
                  "subregions": [
                    {
                      "id": 110,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 111,
                      "name": "Terminal 2",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 48,
                  "name": "Ribeira",
                  "subregions": [
                    {
                      "id": 112,
                      "name": "Dom Luís I Bridge",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 113,
                      "name": "Clérigos Tower",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        {
          "id": 5,
          "name": "Netherlands",
          "subregions": [
            {
              "id": 17,
              "name": "Amsterdam",
              "subregions": [
                {
                  "id": 49,
                  "name": "Amsterdam Airport Schiphol",
                  "subregions": [
                    {
                      "id": 114,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 115,
                      "name": "Terminal 2",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 50,
                  "name": "Jordaan",
                  "subregions": [
                    {
                      "id": 116,
                      "name": "Anne Frank House",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 117,
                      "name": "Westerkerk",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            },
            {
              "id": 18,
              "name": "Rotterdam",
              "subregions": [
                {
                  "id": 51,
                  "name": "Rotterdam The Hague Airport",
                  "subregions": [
                    {
                      "id": 118,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 52,
                  "name": "Delfshaven",
                  "subregions": [
                    {
                      "id": 119,
                      "name": "Delfshaven Harbor",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 120,
                      "name": "Pilgrim Fathers Church",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        {
          "id": 6,
          "name": "Belgium",
          "subregions": [
            {
              "id": 19,
              "name": "Brussels",
              "subregions": [
                {
                  "id": 53,
                  "name": "Brussels Airport",
                  "subregions": [
                    {
                      "id": 121,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 54,
                  "name": "Brussels South Charleroi Airport",
                  "subregions": [
                    {
                      "id": 122,
                      "name": "Terminal 1",
                      "subregions": [

                      ]
                    }
                  ]
                },
                {
                  "id": 55,
                  "name": "European Quarter",
                  "subregions": [
                    {
                      "id": 123,
                      "name": "European Commission",
                      "subregions": [

                      ]
                    },
                    {
                      "id": 124,
                      "name": "Parc Leopold",
                      "subregions": [

                      ]
                    }
                  ]
                }
              ]
            },
            {
              "id": 20,
              "name": "Antwerp",
              "subregions": [
                {
                  "id": 56,
                  "name": "Antwerp Central Station",
                  "subregions": [

                  ]
                },
                {
                  "id": 57,
                  "name": "Grote Markt",
                  "subregions": [

                  ]
                }
              ]
            }
          ]
        }
      ]
    }
    ))
    .unwrap()
});

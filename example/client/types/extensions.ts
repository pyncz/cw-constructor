// Base token extension
export interface Attribute {
  trait_type: string
  display_type?: 'number' | null
  value: string
};

export interface Extension {
  name: string
  description: string
  image: string
  attributes: Attribute[]
};

// Trait token extension
export interface TraitAttribute {
  trait_type: string
  display_type?: 'boost_number' | 'boost_percentage' | null
  value: string
}

export interface TraitExtension {
  name: string
  image?: string | null
  attributes: TraitAttribute[]
}

// Merged extension
export interface MergedAttribute {
  trait_type: string
  display_type?: 'number' | 'decimal' | null
  value: string
}

export interface MergedExtension {
  name: string
  description: string
  images: [string, ...string[]] // 1+ images
  attributes: MergedAttribute[]
}

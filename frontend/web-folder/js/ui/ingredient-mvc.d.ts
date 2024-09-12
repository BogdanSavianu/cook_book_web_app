import { BaseHTMLElement } from 'dom-native';
import { Ingredient } from '../model/ingredient-mco';
declare class IngredientInput extends BaseHTMLElement {
    #private;
    init(): void;
}
declare global {
    interface HTMLElementTagNameMap {
        'ingredient-input': IngredientInput;
    }
}
export declare class IngredientItem extends BaseHTMLElement {
    #private;
    set data(data: Ingredient);
    get data(): Ingredient;
    init(): void;
    refresh(old?: Ingredient): void;
}
declare global {
    interface HTMLElementTagNameMap {
        'ingredient-item': IngredientItem;
    }
}
export {};

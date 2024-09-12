import { BaseHTMLElement, customElement, getChild, getChildren, html } from 'dom-native';
import { Ingredient, ingredientMco } from '../model/ingredient-mco';

@customElement("ingredient-mvc")
class IngredientMvc extends BaseHTMLElement {
  #ingredientInputEl!: IngredientInput;
  #ingredientListEl!: HTMLElement;

  init() {
    let htmlContent: DocumentFragment = html`
      <div class="box"></div>
      <h1>ingredients</h1>
      <ingredient-input></ingredient-input>
      <ingredient-list></ingredient-list>
    `;
    [this.#ingredientInputEl, this.#ingredientListEl] =
      getChildren(htmlContent, 'ingredient-input', 'ingredient-list');

    this.append(htmlContent);
    this.refresh();
  }

  async refresh() {
    let ingredients: Ingredient[] = await ingredientMco.list();
    let htmlContent = document.createDocumentFragment();
    for (const ingredient of ingredients) {
      const el = document.createElement('ingredient-item');
      el.data = ingredient;
      htmlContent.append(el);
    }

    this.#ingredientListEl.innerHTML = '';
    this.#ingredientListEl.append(htmlContent);

  }

}

@customElement("ingredient-input")
class IngredientInput extends BaseHTMLElement {
  #inputEl!: HTMLInputElement;

  init() {
    let htmlContent = html`
      <input type="text" style="width: 100%;" placeholder="What ingredient do you want to add?">
    `;
    this.#inputEl = getChild(htmlContent, 'input');
    this.append(htmlContent);
  }
}

// ingredient-input tag
declare global {
  interface HTMLElementTagNameMap {
    'ingredient-input': IngredientInput;
  }
}

@customElement('ingredient-item')
export class IngredientItem extends BaseHTMLElement {
  #titleEl!: HTMLElement;
  #data!: Ingredient;

  set data(data: Ingredient) {
    console.log("Setting data for IngredientItem:", data); // Debug: Log when data is set
    let oldData = this.#data;
    this.#data = Object.freeze(data);
    if (this.isConnected) {
      this.refresh(oldData);
    }
  }

  get data() { return this.#data; }

  init() {
    let htmlContent = html`
      <c-check><c-ico name="ico-done"></c-ico></c-check>
      <div class="title">ALA BALA</div> <!-- Removed static title -->
      <c-ico name="del"></c-ico>
    `;
    this.#titleEl = getChild(htmlContent, 'div');
    this.append(htmlContent);
    this.refresh();
  }

  refresh(old?: Ingredient) {
    if (old != null) {
      this.classList.remove(`Ingredient-${old.id}`);
      this.classList.remove(old.name);
    }

    // Render the new data
    const ingredient = this.#data;
    if (ingredient) {
      console.log("Rendering ingredient:", ingredient); // Debug: Log the data being rendered
      this.classList.add(`Ingredient-${ingredient.id}`);
      this.classList.add(ingredient.name);
      this.#titleEl.textContent = `${ingredient.name} (${ingredient.quantity})`;
    }
  }
}

//ingredient-item type augmentation
declare global {
  interface HTMLElementTagNameMap {
    'ingredient-item': IngredientItem;
  }
} 

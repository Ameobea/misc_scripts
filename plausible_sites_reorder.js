// ==UserScript==
// @name        Order plausible sites by views in the last 24 hours
// @namespace   Violentmonkey Scripts
// @match       https://plausible.ameo.dev/sites
// @grant       none
// @version     1.0
// @author      Casey Primozic https://cprimozic.net
// @description 8/16/2022, 10:10:00 PM
// ==/UserScript==

window.addEventListener('load', () => {
  const container = document.querySelector('body > main > div > ul');
  const elems = container.children;
  const elemsWithCounts = Array.from(elems).map(elem => {
    const count = +elem.querySelector(
      'li > div.pl-8.mt-2.flex.items-center.justify-between > span > span > b'
    ).innerText;
    return { elem, count };
  });
  // Re-order the elements by count descending
  const orderedElems = elemsWithCounts.sort((a, b) => b.count - a.count);
  // Remove the elements from the DOM
  orderedElems.forEach(({ elem }) => container.removeChild(elem));
  // Add the elements back in the right order
  orderedElems.forEach(({ elem }) => container.appendChild(elem));
})

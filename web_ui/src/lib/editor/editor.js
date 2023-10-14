
export const defaultEditStatus = { type: "paragraph", status: true }
export const setCursorTo = (element, idx) => {
    let r = document.createRange();
    r.setStart(element, idx);
    r.setEnd(element, idx);
    const currentSelection = document.getSelection();
    currentSelection.removeAllRanges();
    currentSelection.addRange(r);
}
export const selectLine = () => {
    const sel = window.getSelection()
    let node = sel.anchorNode
    let range = document.createRange()
    while (node.previousSibling && node.previousSibling.nodeName !== 'BR') {
        node = node.previousSibling
    }
    range.setStartBefore(node)
    while (node.nextSibling && node.nextSibling.nodeName !== 'BR') {
        node = node.nextSibling
    }
    range.setEndAfter(node)
    return range

}
export const setTitle = (range, type) => {
    let style;
    switch (type) {
        case "h1":
            style = "font-size:36px;font-weight:400"
            break;
        case "h2":
            style = "font-size:30px;font-weight:400"
            break;
        case "h3":
            style = "font-size:24px;font-weight:400"
            break;
        case "h4":
            style = "font-size:18px;font-weight:400"
            break;
        case "h5":
            style = "font-size:12px"
            break;
        case "h6":
            style = "font-size:6px"
            break;
            default:
                style="font-size:10px"
                break;

    }
    let ele = document.createElement('p')
    ele.setAttribute('style', style)
    range.surroundContents(ele)
    range.setStartAfter(ele)
    range.setEndAfter(ele)
    let sel = window.getSelection()
    sel.removeAllRanges()
    sel.addRange(range)
}
export const setSelectionAfterThisElement = (container) => {
    const sel = window.getSelection()
    const dir = checkNodeDirection(sel.anchorNode, sel.focusNode)
    let node = dir ? sel.anchorNode : sel.focusNode
    console.log(sel.anchorNode)
    if (node === container) {
        node = container.childNodes[0]
    }
    let r = document.createRange()
    let newTextNode = document.createTextNode("\u00A0")

    while (node.nextSibling) {
        r.setStartAfter(node.nextSibling)
        r.setEndAfter(node.nextSibling)
        node = node.nextSibling
    }
    while (node && node.parentNode && node.parentNode !== container) {
        node = node.parentNode
    }
    console.log(node)
    node.parentNode.insertBefore(newTextNode, node.nextSibling)
    r.setStart(newTextNode, 0)
    r.setEnd(newTextNode, 1)
    sel.removeAllRanges()
    sel.addRange(r)
    // console.log(newTextNode)
}

const h1 = (content) => {
    let ele = document.createElement('h1')
    ele.setAttribute("style", "font-size:36px;font-weight:400");
    ele.innerHTML = content
    return ele
}
const h2 = (content) => {
    let ele = document.createElement('h2')
    ele.setAttribute("style", "font-size:30px;font-weight:200");
    ele.innerHTML = content
    return ele
}
const h3 = (content) => {
    let ele = document.createElement('h3')
    ele.setAttribute("style", "font-size:24px;font-weight:100");
    ele.innerHTML = content
    return ele
}
const h4 = (content) => {
    let ele = document.createElement('h4')
    ele.setAttribute("style", "font-size:24px;font-weight:100");
    ele.innerHTML = content
    return ele
}
const h5 = (content) => {
    let ele = document.createElement('h5')
    ele.setAttribute("style", "font-size:18px;");
    ele.innerHTML = content
    return ele
}
const h6 = (content) => {
    let ele = document.createElement('h6')
    ele.setAttribute("style", "font-size:12px;");
    ele.innerHTML = content
    return ele
}
const bolding = (content) => {
    let ele = document.createElement('strong')
    ele.innerHTML = content
    return ele
}
const italicing = (content) => {
    let ele = document.createElement('italic')
    ele.innerHTML = content
    return ele
}
const under = (content) => {
    let ele = document.createElement('u')
    ele.innerHTML = content
    return ele
}
const through = (content) => {
    let ele = document.createElement('s')
    ele.innerHTML = content
    return ele
}

export const createElement = (txtContent, action) => {

    switch (action) {
        case 'h1':
            return h1(txtContent)
        case 'h2':
            return h2(txtContent)
        case 'h3':
            return h3(txtContent)
        case 'h4':
            return h4(txtContent)
        case 'h5':
            return h5(txtContent)
        case 'h6':
            return h5(txtContent)
        case 'bold':
            return bolding(txtContent)
        case 'italic':
            return italicing(txtContent)
        case 'under':
            return under(txtContent)
        case 'through':
            return through(txtContent)
        default:
            return h1(txtContent)
    }

}
export const replaceAction = (commonAncestor, selection, action) => {
    // whole node
}


export const selectionHandle = (selection) => {
    switch (selection.type) {
        case 'Caret':

            break;
        case 'Range':
            break;
        default:
            { }
            break;
    }
}
export const checkNodeDirection = (baseNode, otherNode) => {
    if (baseNode.compareDocumentPosition(otherNode) & Node.DOCUMENT_POSITION_FOLLOWING) {
        // 正向
        return true
    } else {
        return false
    }
}
export const getCurrentSelection = (container) => {
    const sel = window.getSelection()
    console.log(sel.anchorNode?.nextSibling)
    let ancestor = sel.getRangeAt(0).commonAncestorContainer
    const dir = checkNodeDirection(sel.anchorNode, sel.focusNode)
    if (ancestor === container) {

        return dir ?
            {
                ancestor: ancestor,
                starter: sel.anchorNode,
                ender: sel.focusNode,
                startOffset: sel.anchorOffset,
                endOffset: sel.focusOffset,
                type: sel.type
            } :
            {
                ancestor: ancestor,

                ender: sel.anchorNode,
                starter: sel.focusNode,
                startOffset: sel.focusOffset,
                endOffset: sel.anchorOffset,
                type: sel.type
            }
    } else {
        if (sel.anchorNode === sel.focusNode) {

            return {
                ancestor: ancestor,
                starter: sel.anchorNode,
                ender: sel.anchorNode,
                startOffset: Math.min(sel.anchorOffset, sel.focusOffset),
                endOffset: Math.max(sel.anchorOffset, sel.focusOffset),
                type: sel.type
            }
        } else {

            return dir ?
                {
                    ancestor: ancestor,
                    starter: sel.anchorNode,
                    ender: sel.focusNode,
                    startOffset: sel.anchorOffset,
                    endOffset: sel.focusOffset,
                    type: sel.type
                } :
                {
                    ancestor: ancestor,
                    ender: sel.anchorNode,
                    starter: sel.focusNode,
                    startOffset: sel.focusOffset,
                    endOffset: sel.anchorOffset,
                    type: sel.type
                }
        }

    }
}

export const traverseNodeTree = (entryNode, sel, tag) => {
    const { starter, ender, startOffset, endOffset } = sel;
    console.log(starter, ender)
    let isAfter = starter.compareDocumentPosition(entryNode) & Node.DOCUMENT_POSITION_FOLLOWING
    let isBefore = entryNode.compareDocumentPosition(ender) & Node.DOCUMENT_POSITION_FOLLOWING
    console.log(isAfter, isBefore)
    if (isAfter && isBefore && entryNode !== ender && entryNode.nodeType !== 1) {
        let range = document.createRange()
        range.selectNode(entryNode)
        range.surroundContents(document.createElement(tag))
    }
    if (entryNode === starter && entryNode.nodeType === 3) {
        let range = document.createRange()
        range.setStart(entryNode, startOffset)
        range.setEndAfter(entryNode)
        range.surroundContents(document.createElement(tag))
    }
    if (entryNode === ender && entryNode.nodeType === 3) {
        let range = document.createRange()
        range.setStart(entryNode, 0)
        range.setEnd(entryNode, endOffset)
        range.surroundContents(document.createElement(tag))
    }
    if (entryNode.hasChildNodes()) {
        entryNode.childNodes.forEach(node => {
            traverseNodeTree(node, sel, tag)
        });
    }

}

export const applyStyleToSelection = (sel, tag) => {

    let rootNode = sel.ancestor
    console.log(rootNode)
    console.log(sel.start, sel.ender)
    let range = document.createRange()
    if (rootNode.hasChildNodes()) {
        console.log('has child')
        traverseNodeTree(rootNode, sel, tag)

    } else {
        range.setStart(rootNode, sel.startOffset)
        range.setEnd(rootNode, sel.endOffset)
        let ele = document.createElement(tag)
        range.surroundContents(ele)
    }
}
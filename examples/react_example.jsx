import React, { useState } from 'react';

const ReactComponent = ({ title, items }) => {
  const [active, setActive] = useState(false);

  return (
<div 
     className="react-container"
              data-active={active}
  >
               <header     >
        <h1 
             className="title-header"
   >
      {title}
               </h1>
            <button 
                         onClick={() => setActive(!active)}
                        className={`toggle-btn ${active ? 'active' : ''}`}
                 >
   Toggle State
                 </button>
                    </header>

          <main>
    {items.length > 0 ? (
      <ul className="item-list">
        {items.map((item, index) => (
          <li key={item.id} className="list-item">
            <span>{item.name}</span>
            <button
              onClick={() => console.log('clicked', item.id)}
              disabled={!active}
            >
              Action
            </button>
          </li>
        ))}
      </ul>
    ) : (
      <div className="empty-state">
        <p>No items found.</p>
      </div>
    )}
          </main>
</div>
  );
};

export default ReactComponent;


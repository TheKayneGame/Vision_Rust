function avansrect(org,bw,areamin,areamax,color,width)
%SHOWRECT   Show rectangles around objects (ver 1.0)
%   AVANSRECT(ORG) shows a rectangle in image org, around every object.
%
%   AVANSRECT(ORG,BW) shows a rectangle in image org, around every
%   object in image bw.
%
%   AVANSRECT(ORG,BW,AREAMIN,AREAMAX) shows a rectangle in image org
%   around every object in image bw with an area between areamin 
%   and areamax.
%
%   AVANSRECT(ORG,BW,AREAMIN,AREAMAX,COLOR) as above. The color of the 
%   rectangle is specified as a matrix [R G B], where R, G and B are 
%   scalars ranging from 0 to 1.
%
%   AVANSRECT(ORG,BW,AREAMIN,AREAMAX,COLOR,WIDTH) as above. The default 
%   linewidth is 2.
%

if (nargin < 5)
    width = 2;
end

if (nargin < 5)
    color = [.75 0 0];
end

if (nargin < 4)
    [n m] = size(org);
    areamax = n * m;
end
    
if (nargin < 3)
    areamin = 0;
end

if (nargin < 2)
    bw = org;
end
   
if (nargin > 1)
    if ~islogical(bw)
        bw = bw ~= 0;
    end
end

imshow(org);
[bwl num] = bwlabel(bw);
stats = regionprops(bwl,'Area','BoundingBox');
for i=1:num
    if ((stats(i).Area >= areamin) && (stats(i).Area <= areamax))
        h = rectangle('Position', stats(i).BoundingBox, 'LineWidth', width);
        set(h,'EdgeColor', color);
    end
end

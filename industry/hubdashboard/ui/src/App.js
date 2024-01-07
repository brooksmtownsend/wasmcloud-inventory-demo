import React, { useState, useEffect } from 'react';
import {
  ChakraProvider,
  Box,
  Text,
  VStack,
  Grid,
  theme,
  Button,
  Menu, MenuButton, MenuList, MenuItem,
  Table, Thead, Tbody, Tr, Th, Td, HStack
} from '@chakra-ui/react';
import { ColorModeSwitcher } from './ColorModeSwitcher';
import { Logo } from './Logo';

function App() {
  const [inventoryData, setInventoryData] = useState([]);

  const fetchInventory = () => {
    // Requests rundown from units, then fetches updated inventory
    fetch("/rundown")
      .then((response) => {
        console.dir(response);
        // Fetch data from your API endpoint
        fetch("/inventory")
          .then((response) => response.json())
          .then((data) => setInventoryData(data.flat(Infinity).sort((a, b) => a.unit.localeCompare(b.unit))))
          .catch((error) => {
            console.error("Error fetching data:", error);
          });
      })
      .catch((error) => {
        console.error("Error fetching data:", error);
      });
  };

  const clearInventory = () => {
    fetch("/clear", { method: "DEL" })
      .then((response) => console.dir(response))
      .then((_response) => fetchInventory())
      .catch((error) => {
        console.error("Error fetching data:", error);
      });
  };

  const [selectedItem, setSelectedItem] = React.useState(null);

  const handleMenuItemClick = (itemValue) => {
    setSelectedItem(itemValue);
  };


  useEffect(() => {
    fetchInventory(); // Fetch data when the component mounts
    setInterval(fetchInventory, 2000); // Continually poll for updates
  }, []);

  return (
    <ChakraProvider theme={theme}>
      <Box textAlign="center" fontSize="xl">
        <Grid p={3}>
          <ColorModeSwitcher justifySelf="flex-end" />
          <VStack spacing={4}>
            <Logo h="200px" pointerEvents="none" />
            <Box width="100%">
              <Box overflowX="auto" textAlign="center" mb={4}>
                <Text fontSize="2xl">
                  Hub Dashboard
                </Text>
                <HStack justifyContent="center">
                  <Menu>
                    {({ isOpen }) => (
                      <>
                        <MenuButton isActive={isOpen} as={Button} rightIcon="▼">
                          Select Unit
                        </MenuButton>
                        <MenuList>
                          <MenuItem onClick={() => handleMenuItemClick(null)}>All</MenuItem>
                          {[...new Set(inventoryData.map((i) => i.unit))].map((item) => (
                            <MenuItem key={item} onClick={() => handleMenuItemClick(item)}>{item}</MenuItem>
                          ))}
                        </MenuList>
                      </>
                    )}
                  </Menu>
                  <Button color="red" onClick={() => clearInventory()}>Clear</Button>
                </HStack>
                <Table variant="simple" width="100%" maxWidth="50vw" mx="auto">
                  <Thead>
                    <Tr>
                      <Th>Unit</Th>
                      <Th>Item</Th>
                      <Th textAlign="right">Quantity</Th> {/* Right-align Quantity column */}
                    </Tr>
                  </Thead>
                  <Tbody>
                    {inventoryData.filter((item) => {
                      if (!selectedItem) {
                        return true;
                      } else {
                        return item.unit === selectedItem;
                      }
                    }).map((item) => (
                      <Tr key={item.id}>
                        <Td>{item.unit}</Td>
                        <Td>{item.item_type}</Td>
                        <Td textAlign="right">{item.quantity}</Td> {/* Right-align Quantity cell */}
                      </Tr>
                    ))}
                  </Tbody>
                </Table>
              </Box>
            </Box>
          </VStack>
        </Grid>
      </Box>
    </ChakraProvider >
  );
}

export default App;
